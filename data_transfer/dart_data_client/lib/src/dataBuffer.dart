import 'package:dart_data_client/dart_data_client.dart';

const int DATA_BUFFER_SIZE = 1024;
const int NUM_LODS = 30;
const int MOVE_FRAME_CUTOFF = DATA_BUFFER_SIZE >> 2; // Divide by 4

class LodBuffer {
  int firstSample = 0, lastSample = 0;
  int rateModifier;
  List<double> data = List.filled(1024, 0.0);

  LodBuffer(this.rateModifier);
}

class DataBuffer {
  String datasetName;
  ChannelDescriptor channel;
  Function dataModifiedCallback;
  List<LodBuffer> buffers =
      List.generate(NUM_LODS, (rateModifier) => LodBuffer(rateModifier));

  DataBuffer(this.datasetName, this.channel, this.dataModifiedCallback) {
    for (int i = 0; i < NUM_LODS; i++) {
      getDataFromServer(0, i);
    }
  }

  void getDataFromServer(int firstSample, int rateModifier) {
    if (firstSample < 0) {
      firstSample = 0;
    }
    getSamples(
            datasetName,
            ReadSamplesParams(
                channel, firstSample, firstSample + DATA_BUFFER_SIZE,
                rateModifier: rateModifier))
        .then((data) {
      var buf = buffers[rateModifier];
      buf.firstSample = firstSample;
      buf.lastSample = firstSample + DATA_BUFFER_SIZE;
      buf.data = data;
      dataModifiedCallback();
    });
  }

  double getZoomOffset(double zoom) {
    while (zoom < 1.0) {
      zoom *= 2.0;
    }
    return zoom;
  }

  // Zoom is how many points in [into] per sample point.
  // E.G. a value of 0.1 will give 1 output point every 10 samples.
  // Zoom will be rounded to a power of two.
  // True will be returned if data is immediately available, false otherwise.
  bool getData(List<double> into, double start, double zoom) {
    int rateModifier = 0;
    while (zoom < 1.0) {
      rateModifier += 1;
      zoom *= 2.0;
      start /= 2.0;
    }
    var len = into.length;
    var firstSample = start.floor();
    var lastSample = firstSample + len;
    var buf = buffers[rateModifier];
    if ((buf.firstSample + MOVE_FRAME_CUTOFF > firstSample ||
            buf.lastSample - MOVE_FRAME_CUTOFF < lastSample) &&
        firstSample > MOVE_FRAME_CUTOFF) {
      getDataFromServer(
          firstSample - (DATA_BUFFER_SIZE / 2).floor(), rateModifier);
      into.fillRange(0, len, 0.0);
      return false;
    } else {
      for (int i = 0; i < len; i++) {
        var index = i + firstSample - buf.firstSample;
        if (index < buf.data.length) {
          into[i] = buf.data[index];
        } else {
          into[i] = 0.0;
        }
      }
      return true;
    }
  }
}
