# Weather

## p10: parse-weather-city
- status: active
- kind: language
- trust: 0.9
- support: 5
- provenance: seed

### when
- text: weather in ?city
- not_fact: want_weather()

### then
- assert[working]: want_weather(city=?city)

## p11: parse-weather-no-city
- status: active
- kind: language
- trust: 0.9
- support: 4
- provenance: seed

### when
- text: weather
- not_fact: want_weather()

### then
- assert[working]: asked_slot(goal=weather, slot=city)
- reply: Which city would you like the weather for? (I know Paris, Tokyo, London, and Vilnius.)

## p12: bind-asked-city-for-weather
- status: active
- kind: language
- trust: 0.85
- support: 4
- provenance: seed

### when
- fact: asked_slot(goal=weather, slot=city)
- text: ?city
- not_fact: want_weather()

### then
- retract: asked_slot
- assert[working]: want_weather(city=?city)

## p13: run-weather-tool
- status: active
- kind: reasoning
- trust: 0.92
- support: 5
- provenance: seed

### when
- fact: want_weather(city=?city)
- not_fact: weather_report()
- not_fact: weather_unknown()

### then
- tool: weather(city=?city)

## p14: reply-weather-report
- status: active
- kind: reply
- trust: 0.92
- support: 5
- provenance: seed

### when
- fact: weather_report(city=?city, condition=?cond, temp=?temp)

### then
- reply: Weather in ?city: ?cond, ?temp.

## p15: reply-weather-unknown
- status: active
- kind: reply
- trust: 0.9
- support: 3
- provenance: seed

### when
- fact: weather_unknown(city=?city)

### then
- reply: I don't have a weather fact for ?city yet. You can add a weather_city fact in the Facts tab.
