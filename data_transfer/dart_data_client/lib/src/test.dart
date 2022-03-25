import 'dart:convert';
import 'dart:io';

Future<List<String>> listDatasets() async {
  var req =
      await HttpClient().getUrl(Uri.parse("http://127.0.0.1:9876/datasets"));
  var res = await req.close();
  var text = await res.transform(Utf8Decoder()).join();
  var body =
      (json.decode(text) as List<dynamic>).map((x) => x as String).toList();
  return body;
}

Future<String> getDatasetInfo(String name) async {
  var req = await HttpClient()
      .getUrl(Uri.parse("http://127.0.0.1:9876/datasets/" + name));
  var res = await req.close();
  var text = await res.transform(Utf8Decoder()).join();
  if (res.statusCode >= 400) {
    throw Exception(text);
  }
  // var body = (json.decode(text) as List<dynamic>).map((x) => x as String).toList();
  return text;
}
