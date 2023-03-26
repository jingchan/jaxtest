# %%
import numpy as np
import time
import jax.numpy as jnp
from jax import grad, jit, vmap
from jax import random

key = random.PRNGKey(0)
x = random.normal(key, (10,))
print(x)

# i
size = 3000

start = time.perf_counter()
x = random.normal(key, (size, size), dtype=jnp.float32)
jnp.dot(x, x.T).block_until_ready()  # runs on the GPU
print("Time taken: ", time.perf_counter() - start)

start = time.perf_counter()
x = np.random.normal(size=(size, size)).astype(np.float32)
jnp.dot(x, x.T).block_until_ready()
print("Time taken: ", time.perf_counter() - start)
