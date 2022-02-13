
#
# read the data from the URL and print it
#
from urllib.request import urlopen, Request

import re

import os

def mul10000 (input):
    print('input is ', input)
    parts = re.search(r'([0-9]+)\.([0-9]+)', input)
    if (len(parts.group(2)) >=3) :
        return parts.group(1) + parts.group(2)[0:3]
    else:
        return parts.group(1) + parts.group(2) + '0'*(3-len(parts.group(2)))


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

headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.3'}
total_pages = 10
for p in range(10, total_pages+1):
    reg_url = 'https://etherscan.io/tokens?p=' + str(p)
    print('Requesting webpage', p)
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
    hashes = []
    names = []
    decimals = []
    prices = []

    for line in file1:
        l = line.strip()
        token_line = re.findall(r'href\=\'\/token\/0x[0-9a-fA-F]+\'\>[^\<]+\<', l)
        # token_line = re.findall(r'href\=\'\/token\/0x[0-9a-fA-F]+\'', l)
        if token_line:
            for i in token_line:
                has_hash = re.search(r'href\=\'\/token\/0x([0-9a-fA-F]+)\'\>([^\<]+)\<',i)
                if has_hash:
                    hashes.append(has_hash.group(1))
                    names.append(has_hash.group(2))
            #print (token_line)
            count += len(token_line)
            #print (len(token_line))
    file1.close()
    # print(count)
    # print(hashes)
    # print(names)
    assert(count == len(hashes) and count == len(names))
    for idx in range(len(hashes)):
        print(names[idx] + 'hash:' + hashes[idx])
        reg_url = 'https://etherscan.io/token/0x' + hashes[idx]
        req = Request(url=reg_url, headers=headers) 
        html = urlopen(req).read()
        f = open(log, "w")
        f.write(html.decode("utf-8"))
        f.close()
        file1 = open(log, 'r+')
        price_len = len(prices)
        decimal_len = len(decimals)
        for line in file1:
            l = line.strip()
            # price = re.search(r'\s+\$([0-9\,]+\.[0-9]+).+\|\sEtherscan',l)
            price = re.search(r'on Etherscan shows the price of the Token \$([0-9\,]+\.[0-9]+)', l)
            if price and price_len == len(prices):
                parsed_p = re.sub(r'\,','',price.group(1))
                prices.append(parsed_p)
                # print(parsed_p)
            decimal_check = re.search(r'Decimals\:\<\/div\>',l)
            if decimal_check:
                file1.readline()
                decimal = file1.readline().strip()
                decimals.append(decimal)
                # print(decimal)
        file1.close()
        assert(len(prices) == price_len + 1)
        assert(len(decimals) == decimal_len + 1)
        # break
    prices = [int(float(i)*10000) for i in prices]
    decimals = [int(i) for i in decimals]
    # print(len(prices))
    # print(len(decimals))
    assert(count == len(prices) and count == len(decimals))
    # print(prices)
    # print(decimals)

    out = "parsed_token.txt"
    file2 = open(out, 'a')
    for i in range(count):
        file2.write('//'+names[i]+' '+ str(decimals[i]) +'\n'+'(Address::from_str("' + hashes[i] + '").unwrap(), (U256::from(' + str(prices[i]) + '),U256::from_dec_str("1' + '0'*decimals[i] + '").unwrap())),' + '\n')
    file2.close()
