import 'package:flutter/material.dart';
import 'package:syncfusion_flutter_charts/charts.dart';
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
  double a = 0;
  double xMin = -5;
  double xMax = 5;

  double numpts = 100;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Graphing Page')),
      body: SingleChildScrollView(
          child: Column(children: [
        SfCartesianChart(
          // zoomPanBehavior: ZoomPanBehavior(
          //     enablePinching: true,
          //     enablePanning: true,
          //     enableMouseWheelZooming: true),
          trackballBehavior: TrackballBehavior(
            enable: true,
            activationMode: ActivationMode.singleTap,
            hideDelay: 2000,
            lineWidth: 3,
            lineType: TrackballLineType.vertical,
            markerSettings: const TrackballMarkerSettings(
                shape: DataMarkerType.circle,
                markerVisibility: TrackballVisibilityMode.visible),
            tooltipAlignment: ChartAlignment.near,
            tooltipSettings: const InteractiveTooltip(
              format: '(point.x , point.y)',
            ),
          ),
          primaryXAxis: NumericAxis(
            axisLine: const AxisLine(color: Colors.black, width: 2),
            title: AxisTitle(text: 'X-Axis title'),
            labelStyle: const TextStyle(color: Colors.blueGrey, fontSize: 11),
            minimum: xMin,
            maximum: xMax,
            interval: 1,
          ),
          primaryYAxis: NumericAxis(
            axisLine: const AxisLine(color: Colors.black, width: 2),
            title: AxisTitle(text: 'Y-Axis title'),
            labelStyle: const TextStyle(color: Colors.blueGrey, fontSize: 11),
            minimum: -5,
            maximum: 5,
            interval: 1,
          ),
          title: ChartTitle(text: 'Graph'),
          legend: Legend(isVisible: true, title: LegendTitle(text: 'Plots')),
          series: <ChartSeries>[
            LineSeries<_Graph, double>(
              name: 'Curve Graph',
              dataSource: List.generate(
                      101,
                      (i) => ((i / numpts) * (xMax - xMin) +
                          xMin)) //L_Xmin = 50, L_Xmax = 10
                  .map((x) => _Graph(x, cos(x) * a))
                  .toList(),
              xValueMapper: (_Graph point, _) => point.xval,
              yValueMapper: (_Graph point, _) => point.yval,
            ),
          ],
        ),
        GestureDetector(
            behavior: HitTestBehavior.opaque,
            child: const SizedBox(width: 1000, height: 1000, child: null),
            onHorizontalDragUpdate: (DragUpdateDetails details) {
              if (details.delta.dx < 0) {
                setState(() {
                  xMin++;
                  xMax++;
                });
              } else {
                setState(() {
                  xMin--;
                  xMax--;
                });
              }
            },
            onVerticalDragUpdate: (DragUpdateDetails details) {
              if (details.delta.dy > 0) {
                setState(() {
                  xMin++;
                  xMax--;
                });
              } else if (details.delta.dy < 0) {
                setState(() {
                  xMin--;
                  xMax++;
                });
              }
            }),
        TextField(
            onChanged: (text) {
              setState(() {
                a = double.parse(text);
              });
            },
            //Text
            // controller: _valueController,
            decoration: InputDecoration(
                labelText: 'Type Here',
                labelStyle: const TextStyle(
                  color: Colors.black,
                  fontSize: 20,
                ),
                hintText: 'Input Value',
                hintStyle: const TextStyle(fontSize: 20, color: Colors.red),
                prefixIcon: const Icon(Icons.add, color: Colors.black),
                fillColor: Colors.black12,
                filled: true,
                border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(20)))),
      ])),
    );
  }
}

class _Graph {
  _Graph(this.xval, this.yval);
  final double xval;
  final double yval;
}
