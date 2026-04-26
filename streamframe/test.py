import streamframe as sf

df = sf.StreamFrame(["price"], 3)

df.append({"price": 100.0})
df.append({"price": 200.0})
df.append({"price": 300.0})
df.append({"price": 400.0})

print("=====")

print(df.mean("price"))          
print(df.rolling_mean("price"))  
print(df.last("price"))

print("=====")

print(df.rolling_mean("price"))  
print(df.rolling_std("price"))

df.append({"price": 400})

print(df.rolling_mean("price"))  
print(df.rolling_std("price"))

print("=====")