import asyncio
import python_data_client as pdc


async def main():
    client = await pdc.Client.new()

    res = await client.list_datasets()
    print('The available data sets are:')
    print(res)

    dd = await client.dataset_descriptor('sample')
    print('The channels of dataset "sample" are:')
    print(dd)

    res = await client.read_samples('sample', dd.channels[0], 0, 'avg', 0, 32)
    print('Some data from the first channel is:')
    print(res)

    res = await client.read_samples('sample', dd.channels[0], 4, 'avg', 0, 32)
    print('Some 16x downsampled data is:')
    print(res)

asyncio.get_event_loop().run_until_complete(main())
