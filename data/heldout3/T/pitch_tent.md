---
id: pitch_tent
stage: social
task: pitch_tent
schema: v3
tier: T
---
# Pitch a tent

## requests
- We're camping for the first time this weekend. How do we put the tent up?
- How do you pitch a dome tent properly?
- Can you take me through setting up a two-person tent at a campsite?

## situation
- I have a dome tent with poles, a groundsheet, pegs, a mallet and guy ropes.

## ground truth
```datalog
task(pitch_tent).
domain(pitch_tent, travel).
goal(pitch_tent, shelter).
needs(pitch_tent, groundsheet).
needs(pitch_tent, tent).
needs(pitch_tent, tent_pole).
needs(pitch_tent, tent_peg).
needs(pitch_tent, mallet).
needs(pitch_tent, guy_rope).
takes_time(pitch_tent, 15, 25, minute).
step(pitch_tent, pitch_tent_s1).
action(pitch_tent_s1, choose).
object_of(pitch_tent_s1, spot).
check(pitch_tent_s1, flat_ground).
step(pitch_tent, pitch_tent_s2).
action(pitch_tent_s2, lay).
object_of(pitch_tent_s2, groundsheet).
step(pitch_tent, pitch_tent_s3).
action(pitch_tent_s3, thread).
object_of(pitch_tent_s3, tent_pole).
into(pitch_tent_s3, tent).
step(pitch_tent, pitch_tent_s4).
action(pitch_tent_s4, raise).
object_of(pitch_tent_s4, tent).
step(pitch_tent, pitch_tent_s5).
action(pitch_tent_s5, peg).
object_of(pitch_tent_s5, tent).
uses(pitch_tent_s5, tent_peg).
role(pitch_tent_s5, tent_peg, fasten).
uses(pitch_tent_s5, mallet).
role(pitch_tent_s5, mallet, strike).
setting(pitch_tent_s5, angle, 45, degree).
step(pitch_tent, pitch_tent_s6).
action(pitch_tent_s6, tighten).
object_of(pitch_tent_s6, guy_rope).
purpose(pitch_tent_s6, stable_in_wind).
before(pitch_tent_s1, pitch_tent_s2).
before(pitch_tent_s2, pitch_tent_s3).
before(pitch_tent_s3, pitch_tent_s4).
before(pitch_tent_s4, pitch_tent_s5).
before(pitch_tent_s5, pitch_tent_s6).
```

## aliases
```datalog
task_alias(pitch_tent, "putting up a tent").
task_alias(pitch_tent, "setting up a tent").
task_alias(pitch_tent, "pitching a tent").
alias(spot, "a pitch").
alias(tent_peg, "pegs").
alias(tent_pole, "poles").
alias(guy_rope, "guy ropes").
alias(flat_ground, "flat ground without stones").
alias(stable_in_wind, "it stays up in the wind").
```

## teach
- To pitch a tent you need a groundsheet, the tent, poles, pegs, a mallet and guy ropes.
- Pitching a tent takes 15 to 25 minutes.
- Step 1 of pitching a tent: choose a spot on flat ground without stones.
- Step 2 of pitching a tent: lay the groundsheet.
- Step 3 of pitching a tent: thread the poles into the tent sleeves.
- Step 4 of pitching a tent: raise the tent.
- Step 5 of pitching a tent: peg the tent down with the mallet, the pegs at a 45 degree angle.
- Step 6 of pitching a tent: tighten the guy ropes so it stays up in the wind.

## checklist
- names what is needed, including the tent, poles, pegs and a mallet
- gives the steps in order: choose spot, groundsheet, poles, raise, peg, guy ropes
- says to peg at an angle

## variant
- change: forgot the mallet: knock the pegs in with a flat rock
- request: We left the mallet at home. How do we get the tent pegs in?
- situation: I have the tent, poles, a groundsheet, pegs and guy ropes, and there are rocks around.
- expect: The reply knocks the pegs in with a flat rock instead of the mallet.

```datalog situation
has(user, groundsheet).
has(user, tent).
has(user, tent_pole).
has(user, tent_peg).
has(user, guy_rope).
has(user, rock).
lacks(user, mallet).
```

```datalog difference
swap(pitch_tent_s5, mallet, rock).
extra_need(rock).
no_need(mallet).
```

## variant
- change: no groundsheet: pitch without it on the driest spot
- request: Our tent didn't come with a groundsheet. Can we still pitch it?
- situation: I have the tent, poles, pegs, a mallet and guy ropes.
- expect: The reply skips laying the groundsheet and stresses choosing a dry, flat spot.

```datalog situation
has(user, tent).
has(user, tent_pole).
has(user, tent_peg).
has(user, mallet).
has(user, guy_rope).
lacks(user, groundsheet).
```

```datalog difference
drop(pitch_tent_s2).
no_need(groundsheet).
change(pitch_tent_s1, check, flat_ground, dry_flat_ground).
```
