import json
import math


def read_json():
    f = open('Sensor_Input.json')
    try:
        data = json.load(f)
    except:
        data = None
    f.close()
    return data

def write_json(file, data: float, status: bool, updated: bool):
    dictionary ={
        "Data" : data,
        "Operational" : status,
        "UID" : 8.6,
        "Updated" : updated,
        "x" : 0.0,
        "y" : 0.0
    }

    with open(file, "w") as outfile:
        json.dump(dictionary, outfile)


if __name__ == '__main__':
    time = 0.0
    while True:
        time += 0.0001
        json_data = read_json()
        if json_data == None:
            write_json('Sensor_Input.json', data=0.0, status=False, updated=False)        
            print("Json_Data Not Configed")
  
        elif 'Updated' in json_data and not(json_data['Updated']):
            write_json('Sensor_Input.json', data=(5*math.sin(time)) + 5, status=True, updated=True)
            print("Write sensor data")
        
