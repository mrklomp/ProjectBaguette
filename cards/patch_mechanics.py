import json, pathlib
from collections import OrderedDict

file_in = pathlib.Path("CORE2025.json")
file_out = pathlib.Path("core2025_custom_patched.json")

# Dictionnaire des effets simples reconnus comme keywords
KEYWORD_TYPES = {
    "taunt": "Taunt",
    "rush": "Rush",
    "charge": "Charge",
    "divine_shield": "Divine Shield",
    "lifesteal": "Lifesteal",
    "poisonous": "Poisonous",
    "reborn": "Reborn",
    "stealth": "Stealth",
    "windfury": "Windfury",
    "megawindfury": "Mega-Windfury",
}

# Lecture du fichier original
cards = json.loads(file_in.read_text(encoding="utf-8"))

# Traitement des cartes
for cid, card in cards.items():
    # 1) Initialise mechanics si absent
    mech = set(card.get("mechanics", []))

    # 2) Balaye tous les effets
    effects = card.get("effects", [])
    if effects is None:
        effects = []
    for eff in effects:
        if "keyword" in eff:
            mech.add(eff["keyword"])
        else:
            kw = KEYWORD_TYPES.get(eff.get("type", "").lower())
            if kw:
                mech.add(kw)
                eff["type"] = "keyword_moved"  # balise temporaire pour suppression

    # 3) Trie et ajoute mechanics
    card["mechanics"] = sorted(mech)

    # 4) Supprime les effets devenus obsolètes
    card["effects"] = [eff for eff in effects if eff.get("type") != "keyword_moved"]

# 5) Réordonne les champs proprement (facultatif mais propre)
cards_ordered = OrderedDict()
for cid, card in cards.items():
    ordered = OrderedDict()
    ordered["card_id"]      = card.get("card_id")
    ordered["card_name"]    = card.get("card_name")
    ordered["card_class"]   = card.get("card_class")
    ordered["card_type"]    = card.get("card_type")
    ordered["cost"]         = card.get("cost")
    ordered["set"]          = card.get("set")
    ordered["rarity"]       = card.get("rarity")
    ordered["collectible"]  = card.get("collectible")
    ordered["spell_school"] = card.get("spell_school")
    ordered["rune_cost"]    = card.get("rune_cost")
    ordered["attack"]       = card.get("attack")
    ordered["health"]       = card.get("health")
    ordered["races"]        = card.get("races")
    ordered["mechanics"]    = card.get("mechanics")
    ordered["effects"]      = card.get("effects")
    ordered["text"]         = card.get("text")

    cards_ordered[cid] = ordered

# 6) Écriture du résultat
file_out.write_text(
    json.dumps(cards_ordered, indent=2, ensure_ascii=False),
    encoding="utf-8"
)

print(f"✅ Patched {len(cards)} cartes → {file_out.name}")
