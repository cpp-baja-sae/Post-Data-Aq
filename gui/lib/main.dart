import 'package:dart_data_client/dart_data_client.dart';
import 'package:flutter/material.dart';
import 'package:charts_flutter/flutter.dart';
import 'dart:math';

void main() {
  runApp(_ChartApp());
}

class _ChartApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        theme: ThemeData(primarySwatch: Colors.red), home: _MyHomePage());
  }
}

class _MyHomePage extends StatefulWidget {
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<_MyHomePage> {
  double start = 0.0;
  double scale = 1.0;
  double a = 0;
  DataBuffer? dataBuffer;
  int numpts = 100;
  List<double> dataPoints = List.generate(100, (_) => 0.0);

  _MyHomePageState() {
    getDatasetInfo("sample").then((info) {
      dataBuffer =
          DataBuffer("sample", info.channels[0], () => setState(() {}));
      setState(() {});
    });
  }

  @override
  Widget build(BuildContext context) {
    dataBuffer?.getData(dataPoints, start, 1.0 / scale);
    var zoomOffset = dataBuffer?.getZoomOffset(1.0 / scale) ?? 1.0;
    return Scaffold(
      appBar: AppBar(title: const Text('Graphing Page')),
      body: SingleChildScrollView(
          child: Column(children: [
        GestureDetector(
            behavior: HitTestBehavior.opaque,
            child: SizedBox(
              width: 1000,
              height: 500,
              child: LineChart(
                [
                  Series(
                    id: "primary",
                    data: dataPoints,
                    domainFn: (x, i) =>
                        (i ?? 0) * scale * zoomOffset + start,
                    measureFn: (x, _) => x,
                  )
                ],
                animate: false,
                animationDuration: Duration(milliseconds: 20),
                domainAxis: NumericAxisSpec(
                    viewport: NumericExtents(start, start + numpts * scale)),
              ),
            ),
            onPanUpdate: (DragUpdateDetails details) {
              setState(() {
                start -= scale * details.delta.dx;
                if (start < 0.0) start = 0.0;
                scale *= pow(1.01, details.delta.dy);
              });
            }),
        TextField(
            onChanged: (text) {
              setState(() {
                a = double.parse(text);
              });
            },
            //Text
            // controller: _valueController,
            decoration: const InputDecoration(
              labelText: 'Type Here',
              hintText: 'Input Value',
              prefixIcon: Icon(Icons.add, color: Colors.black),
            ))
      ])),
    );
  }
}

class _Graph {
  _Graph(this.xval, this.yval);
  final double xval;
  final double yval;
}
