# Weather

Known city forecasts and multi-turn weather lookup. Handles both single-turn (`"weather in Paris"`) and slot-filling (`"weather"` → `"Paris"`).

## Known Cities

```karin
weather_city(city="Paris", condition="partly cloudy", temp="18°C")
weather_city(city="Tokyo", condition="clear skies", temp="22°C")
weather_city(city="London", condition="light rain", temp="13°C")
weather_city(city="Vilnius", condition="crisp and sunny", temp="9°C")
```

## Lookup & Reply Rules

Consumes canonical `intent(name="weather", ...)` facts and uses the shared `dialogue/frames.md` slot protocol (checking `prop(entity="user", attr="city")` first if known).

```karin
rule weather-use-known-user-city
when: intent(name="weather_missing_city"), prop(entity="user", attr="city", value=$city), !intent(name="weather")
then: +intent(name="weather", city=$city)

rule weather-need-city
when: intent(name="weather_missing_city"), !prop(entity="user", attr="city"), !need_slot(goal="weather", slot="city")
then: +need_slot(goal="weather", slot="city", prompt="Which city would you like the weather for? (I know Paris, Tokyo, London, and Vilnius.)")

rule weather-from-slot
when: slot_value(goal="weather", slot="city", value=$city), !intent(name="weather")
then: +intent(name="weather", city=$city)

rule run-weather-tool
when: intent(name="weather", city=$city), !weather_report(), !weather_unknown()
then: @weather(city=$city)

rule reply-weather-report
when: weather_report(city=$city, condition=$cond, temp=$temp)
then: => "Weather in $city: $cond, $temp."

rule reply-weather-unknown
when: weather_unknown(city=$city)
then: => "I don't have a weather fact for $city yet. You can add a weather_city fact in the Facts tab."
```
