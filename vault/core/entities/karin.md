# Karin as an entity (claude-opus-5.5, 2026-11-26)

Karin is the entity `karin`. People can stand in relations to her: "Max is your
creator" stores `rel(subj="karin", pred="creator", obj=<Max>)`.

```karin
rule karin-rel-is-speaker
when: intent(name="karin_rel_tell", person=$x, rel=$rel), speaker(entity=$me), named(entity=$me, value=$x)
trust: 2
then:
  ++rel(subj="karin", pred=$rel, obj=$me)
  => "Got it - you're my $rel!"

rule karin-rel-known
when: intent(name="karin_rel_tell", person=$x, rel=$rel), speaker(entity=$me), named(entity=$e, value=$x), $e != $me, $e != "karin"
trust: 2
then:
  ++rel(subj="karin", pred=$rel, obj=$e)
  => "Got it - $x is my $rel."

rule karin-rel-new
when: intent(name="karin_rel_tell", person=$x, rel=$rel), !named(value=$x)
trust: 2
then:
  ++entity(id=$x, kind="person")
  ++display_name(entity=$x, value=$x)
  ++named(entity=$x, value=$x)
  ++rel(subj="karin", pred=$rel, obj=$x)
  => "Got it - $x is my $rel."

rule karin-ask-who-is-your
when: "who is your $rel", rel(subj="karin", pred=$rel, obj=$e), display_name(entity=$e, value=$n), speaker(entity=$me), $e != $me
trust: 2
then: => "My $rel is $n."

rule karin-ask-who-is-your-speaker
when: "who is your $rel", rel(subj="karin", pred=$rel, obj=$me), speaker(entity=$me)
trust: 2
then: => "You are!"

rule karin-ask-who-is-your-unknown
when: "who is your $rel", !rel(subj="karin", pred=$rel), $rel != "name"
then: => "I don't know who my $rel is yet. Tell me, for example, \"Max is your $rel\"."
```
