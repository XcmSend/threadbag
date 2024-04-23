import requests, time

base = "http://localhost:8081"

p2 = "eJzdVMlu2zAU/BeeFZWkKGq5pVmaohuapEibIggoiaZVS2RAUY0dw/9earMdOW6Rorn0Jj6+ZTQzj0sgVcYrEH9fgjwDMUinLJe3Pp3gkAIHmMUdH6L2eKeq3ORKgngJ5iA+QAi7QQBhgGlIQgqp74CFjVPPjSglNIQ+8UMUYbJyQMYMawoLlvDCNj3qm+YlE80Q91U7xq1+ChuVrOTrJFtdmUVhA0v7OVG6PO6bdchii6yYsUwZW8mqipvmrm8xXB1vbt/aX4W256JMVAPl+NNlM4NlmeaVZQP4p+/988MvZj79+i1TH96cy48+0eLq3TVmXqnmJxezH9dMnuozkVydXnxuGpeqlnYuwL49ZbxglglZF4UDUiUNS013tHPu88xMQYwJccCU52JqrwhBFhAveGq4RTdhRcU3hB8mlSpqw/+OeM2EyKXou66cXmsLyba+pSwKJ3AjdhfeVRt71EUYE0i8MIo8D7cjSegSChHEnud7lARPSt233C9jnxCD+aVmsppw3TDaBh+nXHYYt9MqVeuU/84Mvdz/UN6xcwzTojPdgKEdfFYn2xg6N2xggNVqyw0QbtyACd12g9H1fjM8R5c9Tui2nuEIseyPW+9HkYsQgiHxqR04WA9Bi8LzkRfQiIQEBsFL7/yY4q2dH7P+aLm31OwUedllfRZfI4FuHMAzse+BPhhv8LAKO894b8+dlR8Yth9azZpKoTmX7U0TuOp/3kIrmZ5xfSKzJn14KrRW9+NktFo7qMrLu4In/CFvN3VwwfB5MVX3r8Vg7y40AEpYOhPaqpQdqULpNbLWC2ldsaTJayqffs8ORp5eU7Pz6g3cjAv+B2puVr8Ad8h8jw=="

payload = "eJzdVtty2zYQ/Rc8SyxxI0C9xXYcN03Tie3EuUzGA5IQxYgkHJCq7Xj0712KF0mUGEcz7rTjN2KBXSzOOTzAA8pNpAs0+fKAkghNUDhTSX7Nw3AaMDRC5f2NbqMwvDFFUiYmR5MHdIcmYyqJgwn2fC5dQj1KR+gewkw6kvoeJa7LfCKkx5YjFKlSVXmpCnQKNY+bmkmm4moP57fVLk7xdwzRXGW6WwTZRXmfQuABPqfGZidNsbqxCVJFocuzRQCZq89qrilx8tdlG/29OiGFUXGfBSZtJqGkiiKrC4AB8bPoXL0vZpevzLtTVZrX4ir2L+axOHpz/G36Wn3gH/zvn96a7+fix/QjP6pKZ2aRw46IwCDSqQII8kWajlBo8lKFZT2EbW6TqJyhCWFshGY6iWcwxRiGfnSqw1JDe1OVFhrKWBXHSR53gRb5F0Fh0kWpD2dgOWooho6g0HVIsWB8zXEd3kMypw7lgvgSCy6xxP5qC8y547lMCsyp8DEVYg/HTc1h/poFE3R3aVVeTLWtAF0Ft5dc1k1uLivMwob6Zyro+H46freUs0U3nKxUNq6113Y0XxQqU9erbmbbjdUS2ejs9A0/f/G+vJt9/BSZP1+d5285s/HVH5+Jopm5e3kx//ZZ5af2LA6uTi/eoeVyQ1Kuu5YUYd4eSQ0p6BB6t1XZKao2DY9jJvWjpuF7nkOkdJnrMeF7hNSKla7DuZQ+9RlziXDpf2gZfZqeyiFqyp/IIgb4PATen9KJOZaRepROMBcHugWFuJDBXMnqDbF0KCOYC18IQnywpn+b0CQvta2g3SW0nTox5Sa32K1+m+HLYPPn509n7gPMHYbkAHetuUeYKvq4uXtEOAKE4bqSyFYs2HMY8VwMUdjXY/h/YO19ctf0DdPVJ3bXnvddGDu+fOj//qu+XNrFsBgOIKanhK8jpKN46E037t/+LdQ7L78GrJ3nQks3fFgzrzJjq3W+mqkCV83JobVM2bm2L/OoWt4q0Vpz21+Ml51SiyS7SXWgfyQrJbRyaz8vZub2KG7Bq0NtQ4EK57EFAUTHJjW262wlzBBu4qBaV2XufwuNe8bXQbPzYmqx6Sc8J2g2zzbu20pPNeu7oq+aLuE5QbN1uHHv9bOjms6Kt1WzTngO0Hxd/gOReYZ/"

print("Creating scenario..")
xx = requests.post(base+"/saveUrl", json={"url": p2})
print("Response: ")
print(xx.text)
s = xx.json()
scenario_id = s.get('shortUrl', 'not found')
print("Scenario id saved as:", scenario_id)

print("Starting job: ")
js = requests.post(base+"/job/start", json={"id": scenario_id})
print("start job response:", js.json())
print("sleeping for 10 seconds.. ")
time.sleep(10)
print("Quering logs for {scenario_id:}")
logs = requests.post(base+"/scenario/worker/logs", json={"id": scenario_id})
print("Logs returned: ", logs.text)
