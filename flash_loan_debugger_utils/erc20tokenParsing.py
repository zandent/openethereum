
#
# read the data from the URL and print it
#
from urllib.request import urlopen, Request

import re

import os

idx = 0
ret = "erc20_result.txt"
if os.path.exists(ret):
    os.remove(ret)
file2 = open(ret,"a")
token_trans_count = 0

# open a connection to a URL using urllib
# print(requests.get(url = 'http://etherscan.io/tx/'+tx.hex()).text)
# print ('https://etherscan.io/tx/'+tx.hex())
# webUrl  = urllib.request.urlopen('http://etherscan.io/tx/'+tx.hex())

headers = {'User-Agent': 'Mozilla/5.0'}
reg_url = 'https://etherscan.io/tokens'
req = Request(url=reg_url, headers=headers) 
html = urlopen(req).read() 


#get the result code and print it
log = "token.html"
f = open(log, "w")
f.write(html.decode("utf-8"))
f.close()

# Opening file
file1 = open(log, 'r')
count = 0


# file2.write(reg_url+'\n')
# # Using for loop
# for line in file1:
#     #count += 1
#     #print("Line{}: {}".format(count, line.strip()))
#     l = line.strip()
#     token_trans_line = re.search(r'Tokens\sTransferred\:', l)
#     if token_trans_line:
#         tmp_arr = re.split(r'Tokens\sTransferred\:', l)
#         new_tmp = tmp_arr[1]
#         #XXXXXXToXXXXXX
#         tmp1 = re.split(r'From\<\/.\>', new_tmp)
#         del tmp1[0]
#         #print(tmp1)
#         k=0
#         for i in tmp1:
#             #print(i)
#             tmp2 = re.split(r'To\<\/.\>', i)
#             #tmp2[0]: XXXXX tmp2[1]: XXXXXXForXXXXX
#             sender = re.findall(r'\>[^\<]+\<\/span\>\<\/.\>',tmp2[0])
#             sender = re.sub(r'\<\/span\>\<\/.\>','',sender[0])
#             sender = re.sub(r'\>','',sender)
#             # print(senders)
#             # sender = senders[0]
#             # length = len(senders[0])
#             # for j in senders:
#             #     if len(j) < length:
#             #         sender = j
#             receiver = re.findall(r'\>[^\<]+\<\/span\>\<\/.\>',tmp2[1])
#             receiver = re.sub(r'\<\/span\>\<\/.\>','',receiver[0])
#             receiver = re.sub(r'\>','',receiver)

#             tmp3 = re.split(r'For\<\/.\>', tmp2[1])
#             #tmp3[1]: XXXXXXXXX
#             amt = re.findall(r'\>[\.\d\,\s]+\<\/span\>',tmp3[1])
#             #print(tmp3[1])
#             tkn = re.findall(r'\>[\(\)\s\w\$\-\.]+\<\/.\>\<\/div\>',tmp3[1])
#             # receiver = receivers[0]
#             # length = len(receivers[0])
#             # for j in receivers:
#             #     if len(j) < length:
#             #         receiver = j
#             amt = re.sub(r'\<\/span\>','',amt[0])
#             amt = re.sub(r'\>','',amt)
#             amt = re.sub(r'\s','',amt)
#             if tkn:
#                 tkn = re.sub(r'\<\/.\>\<\/div\>','',tkn[0])
#                 tkn = re.sub(r'\>','',tkn)
#             else:
#                 tkn = "N/A"
#             print("Step:",k)
#             k=k+1
#             print("From",sender,"To",receiver,"For",amt,tkn)
#             file2.write("Step: "+str(k)+"\n"+"From "+sender+" To "+receiver+" For "+amt+' '+tkn+'\n')
#             #print(tmp3[1])
#         token_trans_count = token_trans_count + 1
#         break
    
# # Closing files
# file1.close()
# idx=idx+1
# print(idx,'/',len(block['transactions']))
# file2.write(str(idx) + '/' + str(len(block['transactions']))+'\n') 
# file2.write('Token Transfer: '+ str(token_trans_count)+'\n') 

# file2.write('\nToken Transfer: ' + str(token_trans_count) +'\nNormal Transfer: '+str(len(block['transactions'])-token_trans_count))
file2.close()