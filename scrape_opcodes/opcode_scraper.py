from bs4 import BeautifulSoup as bs

html_doc = open('opcodes_table.htmldata').read()

soup = bs(html_doc, 'html.parser')

opcodes = soup.find_all('td', {'class': 'opcode'})

output = ""

for opcode in opcodes:
    start_byte = int(opcode['data-index'])
    start_byte = hex(start_byte)

    inner = opcode.find('div')
    if inner == None:
        continue

    opcode_data = inner.find('div')
    if opcode_data == None:
        continue

    opcode_data = str(opcode_data).replace('<div>', '').replace('</div>', '')
    opcode_data = opcode_data.split('<br/>')[:-1]
    arg_count = int(opcode_data[1].split(' ')[0]) - 1
    opcode_name = opcode_data[0]
    

    output += f'opcode!("{opcode_name}", {start_byte}, {arg_count}),\n'
  
with open('output.txt', 'w') as f:
    f.write(output)