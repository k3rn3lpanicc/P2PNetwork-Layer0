import socket            
 
s = socket.socket()          
port = 48134                  
s.connect(('127.0.0.1', port))
message = "{'command': 'sendMessage','pPacketCommand':0,'pPacketPayLoad':'salam'}"
s.send(message.encode())
while(True):
    message = s.recv(1024).decode()
    if (not message):
        break;
    print(message)
