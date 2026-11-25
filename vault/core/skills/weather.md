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

```karin
rule parse-weather-city
when: "weather in $city", !want_weather()
then: +want_weather(city=$city)

rule parse-weather-no-city
when: "weather", !want_weather()
then:
  +asked_slot(slot="city", goal="weather")
  => "Which city would you like the weather for? (I know Paris, Tokyo, London, and Vilnius.)"

rule bind-asked-city-for-weather
when: asked_slot(slot="city", goal="weather"), "$city", !want_weather()
then:
  -asked_slot
  +want_weather(city=$city)

rule run-weather-tool
when: want_weather(city=$city), !weather_report(), !weather_unknown()
then: @weather(city=$city)

rule reply-weather-report
when: weather_report(city=$city, condition=$cond, temp=$temp)
then: => "Weather in $city: $cond, $temp."

rule reply-weather-unknown
when: weather_unknown(city=$city)
then: => "I don't have a weather fact for $city yet. You can add a weather_city fact in the Facts tab."
```
