import 'package:dart_data_client/dart_data_client.dart';
import 'package:dart_data_client/src/test.dart';

void main() async {
  // listDatasets().then((value) => print(value));
  var info = await getDatasetInfo("sample");
  var source = ReadSamplesParams(info.channels[0], 1000, 1005);
  // var samples = await getSamples("sample", source);
  var samples = await getFilteredSamples(
      "sample", ReadFilteredSamplesParams(source, 0.2));
  print(samples);
}
