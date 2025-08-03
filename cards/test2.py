import json

with open("CORE2025.json", encoding="utf-8") as f:
    data = json.load(f)

battlecry_types = {}

for card in data.values():
    effects = card.get("effects")
    if isinstance(effects, list):
        for effect in effects:
            # Gestion du trigger battlecry dans la racine ou dans extra
            trigger = effect.get("trigger")
            if not trigger and isinstance(effect.get("extra"), dict):
                trigger = effect["extra"].get("trigger")
            if trigger == "battlecry":
                effect_type = effect.get("type")
                keys = tuple(sorted(effect.keys()))
                if effect_type not in battlecry_types:
                    battlecry_types[effect_type] = set()
                battlecry_types[effect_type].add(keys)

print("Types de battlecry trouvés et structures de champs :\n")
for typ, variants in battlecry_types.items():
    print(f"- {typ}")
    for struct in variants:
        print(f"    {struct}")
