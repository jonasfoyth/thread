Problem: 
`Suppose there are n passenger
threads and a car thread. The passengers repeatedly wait to take
rides in the car, which can hold C passengers, where C < n. The
car can go around the tracks only when it is full.

- Passengers should invoke board and unboard.
- The car should invoke load, run and unload.
- Passengers cannot board until the car has invoked load.
- The car cannot depart until C passengers have boarded.
- Passengers cannot unboard until the car has invoked unload.`

Requisitos:

Resolva esse problema para m carros, sendo que m*C < n.
Considere que: 

- C > 0
- n >= C+1
- m > 0 e m < 6