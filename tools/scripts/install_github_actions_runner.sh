# Create a folder
mkdir actions-runner
cd actions-runner
curl -o actions-runner-linux-x64-2.308.0.tar.gz -L https://github.com/actions/runner/releases/download/v2.308.0/actions-runner-linux-x64-2.308.0.tar.gz
tar xzf ./actions-runner-linux-x64-2.308.0.tar.gz

# Create the runner and start the configuration experience
./config.sh --url https://github.com/ConnorJLatham/blackhole --token AG722FAVE34DC5UUIX5JCQ3E4DCZC# Last step, run it!
./run.sh
