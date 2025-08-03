import json

with open("CORE2025.json") as f:
    data = json.load(f)

for card_id, card in data.items():
    effects = card.get("effects")
    if not effects:
        continue
    for eff in effects:
        trigger = eff.get("trigger") or eff.get("extra", {}).get("trigger")
        if trigger == "battlecry":
            print(card_id, card.get("card_name"), eff)
