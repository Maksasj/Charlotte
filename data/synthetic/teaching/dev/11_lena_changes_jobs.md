# Dialogue: Lena changes jobs
domain: work

## script
- Lena is a nurse.
- Lena works at Northwind.
- Lena's salary is low.
- Lena is tired.
- Does Lena work at Northwind? => yes
- What is Lena's salary? => contains: low
- meh
- Lena doesn't work at Northwind anymore.
- Does Lena work at Northwind? => no
- Did Lena work at Northwind? => contains: used to
- Lena works at Globex.
- Does Lena work at Globex? => yes
- If someone is a nurse then they help patients.
- Does Lena help patients? => yes
- Why? => contains: nurse
- Is Lena happy? => unknown

## expect facts
- is_a(lena, nurse)
- -work_at(lena, northwind)
- work_at(lena, globex)
- property(lena, salary, low)
- feels(lena, tired)
- help(lena, patient)

## expect rules
- help(A, patient) :- is_a(A, nurse).

## ground truth
- is_a(lena, nurse)
- is_a(lena, person)
- -work_at(lena, northwind)
- work_at(lena, globex)
- property(lena, salary, low)
- feels(lena, tired)
- help(lena, patient)
