import protos.person_pb2 as person_proto

nice = person_proto.Person()
nice.name = "jeff"
nice.height = 23

print(nice)