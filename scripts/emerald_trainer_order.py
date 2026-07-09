# Script used to retreive the order of all the trainers of
# pokemon emerald from https://pokemonlp.fandom.com/wiki/Appendix:Pok%C3%A9mon_Emerald_Walkthrough
# and map it to https://github.com/rh-hideout/pokeemerald-expansion/blob/master/src/data/trainers.party
# to get a configuration file for generating documentation.

import bs4
import dataclasses
from dataclasses import dataclass
import re
from typing import Optional
import rich
import json


@dataclass
class Pokemon:
    species: str
    level: int


@dataclass
class WalkthroughTrainer:
    type: str
    name: str
    location: str
    party: list[Pokemon]


def extract_trainers_from_html(html: str) -> list[WalkthroughTrainer]:
    soup = bs4.BeautifulSoup(html, "html.parser")

    nodes = []

    for node in soup.find_all("table"):
        if node.has_attr("width") and node.attrs["width"] == "225px":
            nodes.append(node)

    result = []

    for node in nodes:
        all_tr = node.tbody.find_all("tr")
        trainer_type = all_tr[0].th.small.a.span
        if trainer_type.sup is not None:
            trainer_type = "Pkmn Trainer"
        else:
            trainer_type = trainer_type.get_text()

        trainer_name = all_tr[1].th.big.span.get_text()

        location = ""
        try:
            if all_tr[3].th.a is not None:
                location = all_tr[3].th.a.span.get_text()
            elif all_tr[3].th.span is not None:
                location = all_tr[3].th.span.span.get_text()
            else:
                location = all_tr[3].th.get_text()
            if (
                trainer_name == "Steven" and location == "Mossdeep Space Center"
            ):  # Because not a trainer in Mossdeep
                continue

            trainer = WalkthroughTrainer(trainer_type, trainer_name, location, [])

            pokemon_table = node.parent.parent.parent.parent.parent.parent.parent.parent.parent.tbody

            pokemon_table = pokemon_table.find_all("tr", recursive=False)[1].td
            for node in pokemon_table.find_all("table", recursive=False):
                if node.tbody is not None:
                    for line in node.tbody.find_all("tr", recursive=False):
                        for card in line.find_all("td", recursive=False):
                            pkmn_species = card.find(
                                "a", recursive=False
                            ).b.span.get_text()
                            pkmn_level = (
                                card.find("small", recursive=False)
                                .find("span", recursive=False)
                                .get_text()
                            )
                            pokemon = Pokemon(pkmn_species, pkmn_level)
                            trainer.party.append(pokemon)

            result.append(trainer)
        except Exception as e:
            rich.print(e)
            result.append(all_tr[3])

    return result


@dataclass
class PartyFileTrainer:
    id: str
    type: str
    name: str
    party: list[Pokemon]


def parse_trainers_party_file(content: str) -> list[PartyFileTrainer]:
    trainer_delim = r"=== (?P<id>[A-Z0-9_]+) ===\n(?P<details>(?:[\w: &/]+\n)+)\n+(?P<mons>(?:[\w: /@\-\n]+)*)"
    trainer_details = r"(?:(?:Name: ?(?P<name>[\w &]+)?\n?)|(?:Class: (?P<class>[\w ]+)\n?)|(?:Pic: (?P<pic>[\w ]+)\n?)|(?:^Gender: (?P<gender>[\w ]+)\n?)|(?:Music: (?P<music>[\w ]+)\n?)|(?:Items: (?P<items>[\w /]+)\n?)|(?:Double Battle: (?P<double_battle>[\w ]+)\n?)|(?:AI: (?P<ai>[\w ]+)\n?)|(?:Mugshot: (?P<mugshot>[\w ]+)\n?)|(?:Starting Status: (?P<starting_status>[\w ]+)\n?))+"
    pokemons_details = r"(?P<species>[\w-]+)(?: (?:\((?P<gender>[MF])\))? ?(?:@ (?P<item>[\w ]+)))?\n(?:(?:Level+: (?P<level>[0-9]+\s*))\n|(?:Happiness+: (?P<happiness>[0-9]+\s*))\n|(?:Ability: (?P<ability>[\w ]+\s*))\n|(?:Tera Type: (?P<tera_type>[\w]+\s*))\n|(?:EVs: (?P<effort_values>[\w/ ]+\s*))\n|(?:IVs: (?P<individual_values>[\w/ ]+\s*))\n|(?:Shiny: (?P<Shiny>[\w]+\s*))\n|(?:Ball: (?P<Ball>[\w]+\s*))\n|(?:(?P<nature>[\w]+) Nature[\s]*\n))+(?:- (?P<move_1>[\w\- ]+)\n?)?(?:- (?P<move_2>[\w\- ]+)\n?)?(?:- (?P<move_3>[\w\- ]+)\n?)?(?:- (?P<move_4>[\w\- ]+)\n?)?"
    result = []

    for trainer in re.finditer(trainer_delim, content):
        details_match = re.match(trainer_details, trainer.group("details"))
        pokemons = []

        for mon in re.finditer(pokemons_details, trainer.group("mons")):
            pokemons.append(Pokemon(mon.group("species"), int(mon.group("level"))))

        result.append(
            PartyFileTrainer(
                trainer.group("id"),
                details_match.group("class"),
                details_match.group("name"),
                pokemons,
            )
        )

    return result


@dataclass
class FinalTrainer:
    id: str
    location: str
    split: str
    sets_lvl_cap: Optional[int]


@dataclass
class FinalResult:
    starter_level_cap: int
    trainers: list[FinalTrainer]

SPLITS = {
    0: "Roxanne",
    1: "Brawly",
    2: "Wattson",
    3: "Flannery",
    4: "Norman",
    5: "Winona",
    6: "Tate and Liza",
    7: "Juan",
    8: "Elite Four",
}

SPLITS_MILESTONE = [
    ("TRAINER_ROXANNE_1", 15),
    ("TRAINER_BRAWLY_1", 19),
    ("TRAINER_WATTSON_1", 24),
    ("TRAINER_FLANNERY_1", 30),
    ("TRAINER_NORMAN_1", 31),
    ("TRAINER_WINONA_1", 33),
    ("TRAINER_TATE_AND_LIZA_1", 42),
    ("TRAINER_JUAN_1", 46),
    ("TRAINER_WALLACE", 58),
]


def find_from_wt_trainer(
    wt_trainer: WalkthroughTrainer, party_file_trainers: list[PartyFileTrainer]
) -> PartyFileTrainer:
    result = None
    for trainer in party_file_trainers:
        if len(trainer.party) != len(wt_trainer.party):
            continue
        for wt_mon, pf_mon in zip(trainer.party, wt_trainer.party):
            if wt_mon != pf_mon:
                continue
        # if trainer.type.lower() != wt_trainer.type.lower():
        #     continue
        if (
            trainer.name == "KYLA" and wt_trainer.name == "Nicole"
        ):  # ???? Why is she named nicole in the walkthrough but kyla in the party file
            result = trainer
        elif trainer.name == "KATIE" and wt_trainer.name == "Kevin": # ??? Again ??
            result = trainer
        elif trainer.name == "TATE&LIZA" and wt_trainer.name == "Tate & Liza":
            result = trainer
        elif trainer.name.lower() != wt_trainer.name.lower():
            continue

        result = trainer
        break

    if result is None:
        rich.print(f"NOT FOUND: {wt_trainer}")

    return result


def final_trainer_order(
    walkthrough_trainers: list[WalkthroughTrainer],
    party_file_trainers: list[PartyFileTrainer],
) -> list[FinalTrainer]:
    current_split = 0
    result = []

    print(len(walkthrough_trainers))

    for wt_trainer in walkthrough_trainers:
        if not isinstance(wt_trainer, WalkthroughTrainer):
            continue
        pf_trainer = find_from_wt_trainer(wt_trainer, party_file_trainers)
        final_trainer = FinalTrainer(
            pf_trainer.id, wt_trainer.location, SPLITS[current_split], None
        )
        if pf_trainer.id == SPLITS_MILESTONE[current_split][0]:
            rich.print(SPLITS_MILESTONE[current_split])
            current_split += 1
            if len(SPLITS_MILESTONE) > current_split:
                final_trainer.sets_lvl_cap = SPLITS_MILESTONE[current_split][1]

        result.append(final_trainer)

    return result

def encode_value(x):
    if dataclasses.is_dataclass(x):
        return dataclasses.asdict(x)

    return x


if __name__ == "__main__":
    walkthrough_trainers = []
    party_file_trainers = None
    with open("../pokeemerald-expansion/src/data/trainers.party") as f:
        party_file_trainers = parse_trainers_party_file(f.read())

    for i in range(1, 23):
        with open("walkthrough/walkthrough_{}.html".format(i)) as f:
            t = extract_trainers_from_html(f.read())
            walkthrough_trainers += t

    print(
        f"Walkthrough trainer count: {len(walkthrough_trainers)}; trainers party count: {len(party_file_trainers)}"
    )

    final_result = FinalResult(starter_level_cap=15, trainers=final_trainer_order(walkthrough_trainers, party_file_trainers))

    rich.print(final_result)

    result = json.dumps(final_result, default=encode_value)

    with open("out.json", "w") as f:
        f.write(result)

