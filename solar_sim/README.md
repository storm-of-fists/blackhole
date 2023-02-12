# sol_sim
Just a lil side project trying to sim the solar system with the boys.

# a lil todo to set this all up properly:
1. pull this down from github
2. go into folder and run make venv (this will install the venv and all requirements)
3. source venv/bin/activate to activate your virtual environment in your terminal
4. now you should have the virtual environment started and youll see (venv) in your terminal
5. the run make data in order to pull down the ephemeris data from jpl
6. then in order to run the notebooks, start up the jupyter notebook server/kernel and you should be good

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