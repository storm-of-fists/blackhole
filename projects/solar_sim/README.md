# sol_sim
Just a lil side project trying to sim the solar system with the boys.

# some things that would be cool to do
## organization
- add a ephemeris data manager to make it easier to get data
- abstract the different forcing elements in the system instead of all of it living in acc_rel
## physics
- improve accuracy by adding in other physical effects like solar wind
- take into account general relativity
- add better logic so planets dont fly out of the system
## features/data vis
- notebook for plotting errors over time
- animated planetary motion
## solvers
- generalize the solvers so you can simply specify a Butcher Table and have it work
- add implicit solvers
- use solvers that are energy conserving
## data validation
- proper testing for all these different components/data managers/equations