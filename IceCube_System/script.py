import json



def read_json():
    f = open('Sensor_Input.json')
    data = json.load(f)
    f.close()
    return data

def write_json(file, data, status):
    dictionary ={
        "Data" : data,
        "Operational" : status,
        "UID" : 8.6,
        "Updated" : True,
        "x" : 0.0,
        "y" : 0.0
    }

    with open(file, "w") as outfile:
        json.dump(dictionary, outfile)


if __name__ == '__main__':
    while True:
        if 'Updated' in read_json() and not(read_json()['Updated']):
            data = 5.0
            status = True
            write_json('Sensor_Input.json', data, status)
