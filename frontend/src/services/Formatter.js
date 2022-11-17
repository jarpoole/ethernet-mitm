const units = [
  "B/s",
  "KB/s",
  "MB/s",
  "GB/s",
  "TB/s",
  "PB/s",
  "EB/s",
  "ZB/s",
  "YB/s",
];

class Formatter {
  static formatDataArray = (dataArray, interval) => {
    return dataArray.map((dataElement) => ({
      id: dataElement.id,
      data: Formatter.formatDataArrayElement(dataElement.data, interval),
    }));
  };

  static formatDataArrayElement = (data, interval) => {
    return data.map((y, i) => ({
      x: i * 1,
      y: y,
    }));
  };

  static formatBytes = (rawBytes, decimals = 0) => {
    // Solution from https://stackoverflow.com/questions/15900485/correct-way-to-convert-size-in-bytes-to-kb-mb-gb-in-javascript
    if (!+rawBytes) return `0 ${units[0]}`;

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;

    const i = Math.floor(Math.log(rawBytes) / Math.log(k));

    return `${parseFloat((rawBytes / Math.pow(k, i)).toFixed(dm))} ${units[i]}`;
  };

  static formatPackets = (numberOfPackets) => {
    if (numberOfPackets === 1) {
      return "1 packet";
    }
    return `${numberOfPackets} packets`;
  };
}

export default Formatter;
