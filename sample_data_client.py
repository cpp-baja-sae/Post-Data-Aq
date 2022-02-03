import asyncio
import python_data_client as pdc

async def main():
    client = await pdc.Client.new()
    res = await client._send('{"ListDatasets": null }')
    print(res)

asyncio.get_event_loop().run_until_complete(main())
