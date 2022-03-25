import 'package:dart_data_client/dart_data_client.dart';

void main() {
  // listDatasets().then((value) => print(value));
  getDatasetInfo("nonexistant").then((value) => print("the value is " + value));
}