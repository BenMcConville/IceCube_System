import json
import math
import sys


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


def exit_script(n: int):
    if n == 400000:
        inp_0 = str(input("Enter if you would like to continue data generation (y/n): "))
        if inp_0 == 'n':
            sys.exit()
        else:
            return 0
    else:
        return n + 1

if __name__ == '__main__':
    time = 0.0
    is_operational = 0
    while True:
        time += 0.0001
        json_data = read_json()
        if json_data == None:
            is_operational = exit_script(is_operational)

            write_json('Sensor_Input.json', data=0.0, status=False, updated=False)        
            print("Json_Data Not Configed")
  
        elif 'Updated' in json_data and not(json_data['Updated']):
            is_operational = 0
            write_json('Sensor_Input.json', data=(5*math.sin(time)) + 5, status=True, updated=True)
            print("Write sensor data")

        else:
            is_operational = exit_script(is_operational)
        
