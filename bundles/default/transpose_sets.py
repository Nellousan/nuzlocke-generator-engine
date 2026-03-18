import json
import sys

if __name__ == "__main__":
    sets = json.load(open(sys.argv[1]))

    for pkmn, format in sets.items():
        transposed_sets = []
        for format_name, format_sets in format.items():
            for set_name, pkmn_set in format_sets.items():
                pkmn_set["format"] = format_name
                pkmn_set["name"] = set_name
                moves = pkmn_set["moves"]
                pkmn_set["moves"] = []
                for move in moves:
                    if type(move) is list:
                        pkmn_set["moves"].append(move)
                    else:
                        pkmn_set["moves"].append([move])

                if "item" not in pkmn_set:
                    pkmn_set["item"] = []
                elif type(pkmn_set["item"]) is not list:
                    pkmn_set["item"] = [pkmn_set["item"]]

                if "nature" in pkmn_set and type(pkmn_set["nature"]) is not list:
                    pkmn_set["nature"] = [pkmn_set["nature"]]
                if "evs" in pkmn_set and type(pkmn_set["evs"]) is not list:
                    pkmn_set["evs"] = [pkmn_set["evs"]]
                if "ivs" in pkmn_set and type(pkmn_set["ivs"]) is not list:
                    pkmn_set["ivs"] = [pkmn_set["ivs"]]

                transposed_sets.append(pkmn_set)
        sets[pkmn] = transposed_sets

    print(json.dumps(sets, indent=2))
