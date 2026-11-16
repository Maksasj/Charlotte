# Dialogue: Party and concert
domain: events

## script
- The party is at the club.
- The concert is at the stadium.
- The party is fun.
- Anna likes the party.
- Bob likes music.
- Bob is excited.
- If someone likes music then they enjoy the concert.
- Does Bob enjoy the concert? => yes
- Why? => contains: music
- lol
- Where is the party? => contains: club
- Is the party fun? => yes
- Is Bob excited? => yes
- Is Anna excited? => unknown
- What do you know about Bob? => contains: music

## expect facts
- located_in(party, club)
- located_in(concert, stadium)
- property(party, quality, fun)
- likes(anna, party)
- likes(bob, music)
- feels(bob, excited)
- likes(bob, concert)

## expect rules
- likes(A, concert) :- likes(A, music).

## ground truth
- located_in(party, club)
- located_in(concert, stadium)
- property(party, quality, fun)
- likes(anna, party)
- likes(bob, music)
- feels(bob, excited)
- likes(bob, concert)
