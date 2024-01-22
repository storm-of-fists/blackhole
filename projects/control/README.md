Why Do This
I see a lot of the same patterns being needed for hardware control having to be re-implemented. I also see a common failure of applications to make their internal data available to outside users for inspection and analysis/processing. So this framework helps to do that for people and structure things so they can get on with implementing their control logic for long-lived software applications. It also helps to make interfacing with these applications easier with a well-defined interface. I think having a system that can do 1-1000 Hz control with hardware generically could be really useful.

Channel / ChannelGroup
Channels are the different formats/specs for how data gets transferred.

Read/Write Tokens
Only a single Write token can be made for a channel within a component, binary, and node, asset, group, …. Something cannot get a WriteToken to an Input, only an Output. This token is the pointer to the data.

Commands
Commands can come in all sorts of shapes and sizes.

Metadata

Telemetry
Backed by a very low overhead protocol, essentially data that changes each control cycle of an application. UDP Based. Read the different types of data. Some simplified form of this could be used, maybe also in TOML along with other info https://flatbuffers.dev/flatbuffers_guide_writing_schema.html. Whatever is defined in the TOML then gets used to generate a flatbuffer.

Restrictions on Flatbuffer Data Types
Strings and byte arrays shall only be allowed in non-control output channels by default (cannot be input for other control components). There shall be an option that marks this as “unsafe” to allow its use in control components/apps. Unions are acceptable as they shall be verified to always account for the largest memory object.

Composite
Composite interfaces are allowed. If the composite interface contains a string at any level, it cannot be used in control. This composition follows the rules of flatbuffers, but is generally not restricted by it.

Telemetry Rate
The telemetry rate for a channel depends on the run rate of the Components this Interface connects. The telemetry rate will be the lower of the two rates.

Metadata
Units
Units can be assigned to a channel or to each field of an object defined in the schema. Every numeric type shall be assigned a unit. Conversions between units can happen automatically. If there is no unit associated with a channel, then it is marked as None.
Tags
Arbitrary tags can be added to channel metadata so that dynamically subscribing channels can query for and subscribe to them.

Telemetry Types
Telemetry channels shall be one of Control Input, Control Output, Non-Control Output, or Human Input. Control Input and Control Output shall be used within other automation for the purposes of controlling a process. A Non-Control Output shall not be an Input of any other component.

Input
Releasable? As in can something else take over the channel?
If releasable, does it start released?
Start value? Defaults to whatever default flatbuffers uses (except structs are constructed with default values, maybe not at first).
If the type is an input, is that input delayed in order to break an execution cycle?
Output
Releasable? As in can something else take over the channel?
If releasable, does it start released?
Start value? Defaults to whatever default flatbuffers uses (except structs are constructed with default values, maybe not at first).
If the type is an input, is that input delayed in order to break an execution cycle?
Is this a Control output? Typically yes if used in control logic, default to yes. Something like high speed telemetry would not be.
Can only be written to 1 time in a event cycle dispatch

Alarm Channel
An alarm channel is a type of telemetry channel that evaluates certain conditions inside the component and then reports if those conditions are occurring. The alarm then has certain metadata associated with it, similar to normal telemetry channels such as
Response description
Conditions that were met that triggered it
Time it occurred last

Calculated Channel
A calculated channel is essentially one that takes one or more channels and calculates a value from them using some transformation (such as addition or multiplication).

Scaling, Inversion, and other Transformations

GRPC
Used for commands and one shot.

Kafka Style Event
Allows for certain shaped objects to be emitted over a bus. Must do a match statement on either side to ingest the message.

JSON/REST
This kind of channel cannot be used for a real time control node.

Custom Flatbuffer
Maybe youd want this if you already have a definition in flatbuffer.

I2C/SPI/CAN

Flow / FlowGroup
Flows consist of multiple channels. Channels can have different flow components meaning GRPC would be on one flow and telemetry might be on other flow(s). Flows are typically numbered corresponding to the PORT number of the machine.

Telemetry channels will be grouped together into custom flatbuffer objects that define the data on the flow. The max size for these flatbuffers will be XXX MB.

The flow/port number can be specified on channels in order to put them in a manually crafted flow. Flows are typically invisible to the end user and are automatically configured.

Component / ComponentGroup / ComponentTemplate
A Component can combine interfaces, again into a flat single interface. This component would contain logic to manipulate the interface before data gets sent out. It has some “run rate” and can run sequences or controllers.

Internal/External Data
The internal/external data is where data can be collected and used by a component to perform some work. This can then be constrained to one of multiple privacy levels: 1. Component 2. Process 3. Asset 4. Asset Group. By default data is accessible to other components in the Process, so privacy level 2. Any data not being accessed at any data level will throw a warning in the compiler to remove it.

The Data can be represented as raw structs in the component, possibly even using flatbuffers to store the data and mutate it inline. This might be tough to create though. Instead we might just need to use flatbuffers for telemetry only and implement a custom state solution.

Default data
In_control of type boolean telemeters if the component is inside a “control” state.
If the state is in control
Uptime in Duration (seconds and nanoseconds) telemeters how long the component has been active for
Start Time in Duration since epoch
Current State
Target State
All modes
Dispatch Information
Last dispatch time
Heartbeat channel, could just be the uptime channel or something, each connecting component would look for it.

Transforms
Data Manipulation
This is where the component can perform certain actions on the data coming from the interface. This includes something like applying transforms. Some things, like scalings (polynomial or otherwise) are only available to certain types (like telemetry). Could maybe do others for things like grpc or whatever. There isnt anything stopping a user from doing these conversions inside the component, but they are just meant to be helpers. If a numeric scaling is applied, the unit can be maintained (being marked as modified/scaled) or even applied.

Access Manipulation
Components can “translate” channel names in the interface connection into different names inside the component. These translated names are then also used elsewhere in the component but not within the binary. The translation can be marked as “public” in order to be seen/used elsewhere within the rest of the binary.

Externalizing data
Can specify at what level an output can be exposed. Either private, component, process, asset, etc. Then if the level is set to private (just used internally), then another component attempting to access and read the data will not be able to.

Rate Limiting
Components could rate limit/downsample data coming from higher speed components within the asset. Maybe they could even apply some sort of transform on the buffer of them. TBD

Static Data
Constant shape/size at runtime. All elements of the data will exist the entire time the program lives. Some values may not get updated, but there will be a value present.

This long lived data could be a memory blob that stores the data, then hands out pointers to each element containing the start and end indices of the data. The data could also be stored within the components themselves (where the interfaces are bound to) for better cache performance. If stored in components, a memory structure containing pointers for all data in the order in which they should be emitted can be added.

Long Lived data is also where the data for the interface lives.

Since much of this data should be telemetry data, it could all be collected into mutable flatbuffer objects at the component level. Then its nice and packed tightly.

Less Network Traffic or Always Telemetering
Components or maybe at the Process level, could specify if telemetry is event based. So you dont emit each cycle if it hasnt changed or hasnt met some “emit” conditions. The emit conditions could be like on change or more complicated, could write your own. This can help reduce network traffic. This would NOT have a concept of “stale” telemetry.

By default, this option would be off and a channel would telemeter to other components on each dispatch cycle of its own. Other components or the component itself would then keep track of the control rate of its inputs and report on the staleness of the telemetry coming into it. This can be part of the “Losing Control” reporting response.

Better Cache or Better Size
Could choose default memory layouts of compacted or better for cache. So you could choose to centralize all the state to reduce its size (everything is a pointer into the centralized state) or have different channels contain the full size of the data locally to improve caching.

Better size would also be important for microcontrollers. Data types of all bit sizes can be made and are always encouraged, whether transmission requirements drive them or not.

Loss Of Control
There are many things that could result in loss of control. One of the biggest indicators there is a problem is stale telemetry. However, other conditions such as a component needing to recycle due to a failed crc or string reset or

Dynamic Data
Can be changed and updated at runtime. Should however be limited in size and reserved so that the space is guaranteed to be there. Could be a setting to reserve memory.

Maybe you can only interact with this data if the process you are running is “non-control”.

Could have helpers for registering other components at runtime, such as “make this rtd component and run it in this one”. This would allow runtime instantiation of structs for something like device mapping. This sort of thing could only be done in “non-control” datas.

Disabled External State
Some channels will need to indicate that they can be disabled under certain conditions, such as when a connection is expected to go offline. They can do this with conditions, either looking at the runtime state or other internal state. Need to determine how they should be disabled within a Process, like if the channel

Component TypesTBD
Components can be really fat with lots of nice features to make them easy to use or slimmed down to fit on microcontrollers. There is no bloat. Thanks templating!

Different component types can be made that offer pre-made selections for different settings. Maybe a group working on microcontrollers needs to write multiple components operating at 100s of Hz. Or 1000s. They could choose to forego some of the niceties that a more “full” component incorporates.

Component States
State machines are pretty easy to reason about. It has some inputs and some outputs and it can change how it calculates those or do different things based on its state. So the entirety of the time a component exists is spent in a state, with either some default behavior or in a user state.

All states can be marked with timeouts. Timeouts should have some kind of action associated with them, such as throwing an error in the binary lifecycle or transitioning to some new state. Most states have a private and user defined. All can be overridden, but the private will need to have its default logic duplicated/performed in some way.

States are marked with the tag “control” or “non-control”. Until all components are in “control” states in a binary, the “in control” flag will not go high. This is a default output.

If the states are asynchronous, they will also be allowed to make use of a “sequence” and a “controller”. The sequence is an asynchronous method that is able to occur over multiple dispatch cycles of the component runtime. The controller runs on each dispatch. Any state can be marked as asynchronous and include a sequence. If not marked with asynchronous, only a controller will be allowed.

Each state can be marked as pass-through or at-least-1-cycle. If marked as pass-through, the process will continue to execute until it is able to complete one dispatch of the run substate for a component. This allows a component to traverse multiple state changes within a dispatch cycle. Marking the state as at-least-1-cycle will mean the run substate, upon encountering a state change, will immediately relinquish control to the rest of the component and process dispatch cycle. Upon the next dispatch cycle, the exit substate will run and the entry and run substates of the next component dispatch are executed. At least 1 cycle is on by default. TBD

The use of malloc should be eliminated or highly restricted within a component, especially its runtime states.

Could auto-generate a state where all inputs and outputs get released so that operators may intervene. Maybe there is even a pause mode where operators can initiate a single cycle dispatch of the component via some command channel. Could be useful for troubleshooting.

Create (Private -> User) (Non-Control)
Get the InitContext while in this state. Can be used to set values for the component.

Start (Private -> User) (Non-Control)
Get the InitContext here as well. Lets you bind to elements of the state maybe? I think most of this could be handled by an interface or the config of some sort.

You can do all sorts of things that are naughty here, like manipulating strings or making byte arrays and generally doing what you want. This state is marked as “non control” since we could wait on the network or other parameters for as long as possible.

Once all components have exited this lifecycle state, the process would be considered ready to engage in control work.

Runtime States (User) (All marked default as Control)
Runtime states can be defined by the user. These states are similar to the lifecycle states, sharing tags such as “control” and timeouts. The runtime states contain 3 substates, entry, run, and exit. Each one can contain their own sequence and controller. Runtime states should typically be well-contained and if other logic is needed, use a new state.

Entry
A substate that is entered every time the component enters the state.

Run
A substate that is entered after entry is completed. On subsequent dispatches, if the state has not changed, this method will be called again.

Exit
A substate that is entered after the target state has changed and the component exits the run substate.

Stop (User -> Private) (Non-Control)
This state can allow one last chance to grab state information from other components prior to being destroyed.

Destroy (User -> Private) (Non-Control)
No more access to external interface components as that data will be stale?

Component Dispatch

Dispatch Modes
The “run rate” in Hz is how many times per second the component will run. The component can also be run on conditionals pertaining to program state.

States
Component dispatch can also be visualized as a state machine, just one that runs every cycle instead of over the whole lifecycle. The design here should aim to localize work to the component as much as possible so that as much data as possible makes its way into CPU cache to improve performance. Minimize having to load other components or at least temporally localize that sort of work before turning the gears on user defined logic internal to the component.

Read Value for Inputs from State Pointers into Component Registers
Could get the value from the non-local state (such as from some other component) and store it locally first before doing any work. This would be auto generated from the interface definition.

User-Defined Work
Do whatever the user wants to do, typically the run substate of the state the component is currently in.

Add Commands to Command Queue?

Component Modes
Components can have modes, which are really just enumerations. This can help spice up the behavior a bit. Maybe a high telemetry rate mode will output high rate telem, but we dont need to go into a specific state for that.

Templating
Components could be templated perhaps, similar to a testcase. So you would just fill in your own states for the template, or add your own layer of private data. So for example, multiple components have the same control scheme, you can then template the component and reuse it with different inputs.

Error Handling
Errors should be handled as much as possible, including the allocation of memory onto the heap. If that is not possible, then some kind of behavior should be defined to put the component into a safe operating mode until the issue can be investigated. Some pre-allocated detailed log/error message could also be built and set. Could also maybe set some error flag high that could inform the error response up to the process level. Sort of like an abort or hold.

Alarm Disabling
Maybe you want to disable alarms for a component that typically monitors for some unsafe condition now that that condition will be manually induced for test. You could turn off alarms manually by setting a certain mode maybe. Then that mode is included as a condition for any alarm running in that component.

Process / ProcessGroup
A Process combines components into a single executable file. This binary also combines the interfaces. This binary is what gets deployed and run. Declaring the interface for the binary allows for it to change internally and be redeployed without needing to redeploy other binaries.

Arguments for this binary can be used in substitutions elsewhere, such as interfaces/components that get attached to the binary. Maybe only make arguments available at “init” time of components.

A binary is designated as either “control” or non-control. Non-control will be subject to more restrictions about what types of channels can be used due to determinism and simplicity. This control boolean percolates through all components of the binary.

Interfaces between processes, at least anything that doesnt share memory directly, will automatically generate loopback/test channels between each other for each type of channel present between them. These test channels shall go in both directions. They shall be capable of being disabled if needed.

Will telemeter a “in control” flag that shows when all components have entered “control” states if this is a control process.

Component Dispatch Order
Component dispatch order should be manually set via the bazel rules. So the components that compose a process will be placed in the order they should be executed in whatever attribute list.

Internal Interface Management
A lot of the interfaces for a component will exist to

Error Handling
As components will be capable of emitting errors to the top level, the process will be able to define top-level behavior. This could be in the form of a function that affects component states (such as setting them all to safe idle states for example). It could also maybe disable the singular component. Or restart it. Or pause execution of the entire process. I think some options are good to offer, but considering that a “default” could be dangerous depending on the scenario the process is used in, the use should need to define it/choose it themselves.

Node / NodeGroup
A Compute Node is the computer that will run Binaries. It can have one or multiple cores. These Binaries should be assigned to cores of the computer/processor to improve determinism.

Asset / AssetGroup
An Asset is a collection of Compute Nodes. It will need to declare a Compute Node as its leader (TBD). An asset can be tagged with a deploy group so that it deploys along with other assets.

An Asset group is a collection of Assets. As with each layer, it assembles the interface such that it is known to external asset groups.
Can have groups of groups of groups
Assembles all asset interfaces together
Must declare a group leader
Assets or asset groups could get human-labeled “hazard tags” that depict the level of danger this asset will be operating with. If its remote and far from humans, maybe the hazard tag is minimal. If itll be right next to them, maybe its critical. TBD

Other Ideas/Uncategorized
Telemetry repeaters will end up being commonplace parts of assets/assetgroups. These will need to take the input from components and make it available to avoid impacting the control loop. These could take “strict” schemas and turn them into “relaxed” interfaces that will accept when channels are missing.

Runtime vs build time flags/sections

Simulating dispatch cycles of components at build time or as part of a smoketest to get a sense of the requirements for running the component. Could it actually run at 500Hz?

Have keyword protection for things like disabled or invalid and instead suggest enabled and valid. Should try to encourage using “positive” indicators.

Would it be possible to include a form of crc’s for chunks of data so that at runtime you could crc the component before its due to run (or even just parts of it) so you dont need a full string reset? Or it reduces the likelihood of a full string reset.

Look at nvidia nsight tools and see what they offer in terms of compute and memory management.
