import json

with open("CORE2025.json", encoding="utf-8") as f:
    templates = json.load(f)

for t in templates.values():
    effects = t.get('effects')
    if effects:
        for e in effects:
            if e.get('type') == 'summon' :
                print(f"{t['card_name']} ({t['card_id']}) : {e}")
