import hashlib
import base64
import time

hashed_password = "pbkdf2_sha256$870000$sHkKiWrJE8kN0gXdkpHMw4$wBQp3tnoICz/cQLhydscEYIBT0VvG5OGFQDJ/I67fpE="
password_user_posted = "hogehoge"

algo, iterations, salt, hash = hashed_password.split('$')
iterations = int(iterations)

start_time = time.perf_counter()

decoded_hash = base64.b64decode(hash)

test_hash = hashlib.pbkdf2_hmac(
    'sha256',
    password_user_posted.encode('utf-8'),
    salt.encode('utf-8'),
    iterations
)

end_time = time.perf_counter()

print(decoded_hash == test_hash)  # Trueなら検証成功
print("{:.2f}ms".format((end_time - start_time) * 1000))