# Dialogue: The chefs' kitchen
domain: food

## script
- Carlos is a chef.
- Yuki is a chef.
- Sam is a chef.
- Sam is vegan.
- If someone is a chef then they eat meat.
- Does Carlos eat meat? => yes
- Why? => contains: chef
- Does Sam eat meat? => yes
- No, Sam doesn't eat meat.
- Does Sam eat meat? => no
- Does Yuki eat meat? => yes
- mmm, tasty
- Yuki loves soup.
- Who loves soup? => contains: yuki
- Does Carlos like soup? => unknown
- Carlos is happy.
- Is Carlos happy? => yes

## expect facts
- is_a(carlos, chef)
- is_a(sam, chef)
- property(sam, diet, vegan)
- eat(carlos, meat)
- eat(yuki, meat)
- likes(yuki, soup)
- feels(carlos, happy)

## expect rules

## ground truth
- is_a(carlos, chef)
- is_a(yuki, chef)
- is_a(sam, chef)
- property(sam, diet, vegan)
- eat(carlos, meat)
- eat(yuki, meat)
- -eat(sam, meat)
- likes(yuki, soup)
- feels(carlos, happy)
