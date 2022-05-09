import 'dart:convert';
import 'package:http/http.dart';

Future<List<String>> listDatasets() async {
  var res = await Client().get(Uri.parse("http://127.0.0.1:9876/datasets"));
  var text = res.body;
  var body =
      (json.decode(text) as List<dynamic>).map((x) => x as String).toList();
  return body;
}

class ChannelDescriptor {
  double sampleRate = 0.0;
  String name = "";
  Map<String, dynamic> typ = new Map();

  ChannelDescriptor(Map<String, dynamic> map) {
    this.sampleRate = map["sample_rate"];
    this.name = map["name"];
    this.typ = map["typ"];
  }
}

class FileDescriptor {
  List<ChannelDescriptor> channels = List.empty(growable: true);

  FileDescriptor(Map<String, dynamic> map) {
    this.channels =
        (map["channels"] as List).map((e) => ChannelDescriptor(e)).toList();
  }
}

Future<FileDescriptor> getDatasetInfo(String name) async {
  var res =
      await Client().get(Uri.parse("http://127.0.0.1:9876/datasets/" + name));
  var text = res.body;
  if (res.statusCode >= 400) {
    throw Exception(text);
  }
  // var body = (json.decode(text) as List<dynamic>).map((x) => x as String).toList();
  return FileDescriptor(json.decode(text));
}

class ReadSamplesParams {
  ReadSamplesParams(ChannelDescriptor channel, this.start, this.end,
      {this.rateModifier = 0, this.downsampleFilter = "avg"}) {
    this.channelTyp = channel.typ;
  }

  Map<String, dynamic> toJson() {
    return {
      'channel': this.channelTyp,
      'rate_modifier': this.rateModifier,
      'downsample_filter': this.downsampleFilter,
      'start': this.start,
      'end': this.end,
    };
  }

  dynamic channelTyp;
  int rateModifier, start, end;
  String downsampleFilter;
}

Future<List<double>> getSamples(String name, ReadSamplesParams params) async {
  var res = await Client().get(Uri(
      scheme: "http",
      host: "localhost",
      port: 9876,
      path: "datasets/" + name + "/samples",
      queryParameters: {'params': json.encode(params.toJson())}));
  var text = res.body;
  if (res.statusCode >= 400) {
    throw Exception(text);
  }
  return (json.decode(text) as List).map((e) => e as double).toList();
}

class ReadFilteredSamplesParams {
  ReadFilteredSamplesParams(this.source, this.relativeCutoff);

  Map<String, dynamic> toJson() {
    return {
      'source': this.source,
      'relative_cutoff': this.relativeCutoff,
    };
  }

  ReadSamplesParams source;
  double relativeCutoff;
}

Future<List<double>> getFilteredSamples(
    String name, ReadFilteredSamplesParams params) async {
  var res = await Client().get(Uri(
      scheme: "http",
      host: "localhost",
      port: 9876,
      path: "datasets/" + name + "/filtered_samples",
      queryParameters: {'params': json.encode(params.toJson())}));
  var text = res.body;
  if (res.statusCode >= 400) {
    throw Exception(text);
  }
  return (json.decode(text) as List).map((e) => e as double).toList();
}
