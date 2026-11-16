# Dialogue: Vehicles and fuel
domain: vehicles

## script
- The bus is a vehicle.
- The truck is a vehicle.
- Sparky is a vehicle.
- Sparky is electric.
- If something is a vehicle then it needs fuel.
- Does the truck need fuel? => yes
- Why? => contains: because; vehicle
- Does Sparky need fuel? => yes
- No, Sparky doesn't need fuel.
- Does Sparky need fuel? => no
- Does the bus need fuel? => yes
- Does the truck need fuel? => yes
- Is Sparky electric? => yes
- hmm, whatever
- The truck is red.
- What colour is the truck? => contains: red
- Is the bus fast? => unknown

## expect facts
- is_a(bus, vehicle)
- is_a(truck, vehicle)
- is_a(sparky, vehicle)
- property(sparky, quality, electric)
- wants(bus, fuel)
- wants(truck, fuel)
- property(truck, colour, red)

## expect rules

## ground truth
- is_a(bus, vehicle)
- is_a(truck, vehicle)
- is_a(sparky, vehicle)
- property(sparky, quality, electric)
- wants(bus, fuel)
- wants(truck, fuel)
- -wants(sparky, fuel)
- property(truck, colour, red)
