# Import the module
import python_rust_showcase

# Instantiatea array
x = python_rust_showcase.PArray(25000)
x.push(1.3)
x.push(3.2)
x.push(1)
y = x.retrieve()
print(f"Push demo: {y}")

x.clear()
x.randomize()
y = x.retrieve()
print(f"Before sort: {y}")

x.sort()
y = x.retrieve()
print(f"After sort: {y}")

print(f"This will be able to handle hundreds of thousands of values due to use of parallelism and optimization techniques.")