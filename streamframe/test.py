import streamframe

df = streamframe.StreamFrame(["price", "volume"])

df.append({"price": 100.0, "volume": 10.0})
df.append({"price": 101.0, "volume": 12.0})
df.append({"price": 102.0, "volume": 8.0})

print(df.mean("price"))
print(df.variance("price"))
print(df.last("price"))