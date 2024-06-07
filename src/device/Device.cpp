//
// Created by imanuel on 20.05.24.
//

#include "device.h"
#include <QDBusConnection>
#include <QJsonArray>
#include <QSettings>
#include <QtLogging>
#include <sys/sysinfo.h>

namespace jewels::device {

float getClockSpeed() {
  auto ok = false;
  auto hz = readFile("/sys/devices/system/cpu/cpu0/cpufreq/base_frequency")
                .toFloat(&ok);
  if (ok) {
    return hz / 1000 / 1000;
  }

  hz = readFile("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
           .toFloat(&ok);
  if (ok) {
    return hz / 1000 / 1000;
  }

  return -1;
}

auto isPartition(const QDir &dir) { return !dir.exists("device"); }

auto getDriveManufacturer(const QDir &dir) -> QString {
  // nvme devices are in /sys/class/nvme/ and /sys/class/block/nvme*
  // but the vendor file is only in /sys/class/nvme/
  // so we need to check if the device is in /sys/class/block/nvme* and if so,
  // we need to change the path
  auto path = dir.path();
  auto nvme_pos = path.indexOf("nvme");
  if (nvme_pos > 0) {
    auto nvme_name = path.sliced(nvme_pos, 5);
    path = path.sliced(0, nvme_pos - 6) + "nvme/" + nvme_name;
  }

  return readFile(path + "/device/vendor");
}

auto getDriveModel(const QDir &dir) -> QString {
  if (dir.exists("device/model")) {
    auto path = dir.filePath("device/model");

    return readFile(path);
  }

  return "Unbekannt";
}

auto getDriveSize(const QDir &dir) {
  if (dir.exists("size")) {
    auto path = dir.filePath("size");

    return readFile(path).toFloat() / ONE_DRIVE_GIB;
  }

  return -1.0f;
}

Device::Device(QObject *parent) : QObject(parent) {
  setDrives();
  setId();
  setHostname();
  setModel();
  setManufacturer();
  setOs();
  setRam();
  setCpu();
  setBios();
  setMainboard();
  setKernel();
}

auto Device::setId() -> void { id = QDBusConnection::localMachineId(); }

auto Device::setHostname() -> void { hostname = QSysInfo::machineHostName(); }

auto Device::setModel() -> void { model = readDmi("product_name"); }

auto Device::setManufacturer() -> void { manufacturer = readDmi("sys_vendor"); }

auto Device::setRam() -> void {
  struct sysinfo info {};
  auto res = sysinfo(&info);
  if (EFAULT == res) {
    ram = 0;
  } else if (res == -1) {
    auto err = errno;
    qWarning() << "Failed to get mem info" << err;
  } else {
    ram = static_cast<float>(info.totalram * info.mem_unit) / ONE_GIB;
  }
}

auto Device::setOs() -> void {
  os = new OperatingSystem();
  os->version = "Unbekannt";

  auto osRelease = readFile("/etc/os-release");
  auto splitOsRelease = osRelease.split("\n");
  foreach (auto line, splitOsRelease) {
    auto splitLine = line.split("=");
    if (splitLine.length() == 2) {
      const auto &key = splitLine.at(0);
      auto value = splitLine.at(1);
      if (key == "NAME") {
        os->name = value.replace("\"", "");
      } else if (key == "VERSION") {
        os->version = value.replace("\"", "");
      }
    }
  }
}

auto Device::setCpu() -> void {
  cpu = new Cpu();
  auto cpuinfo = readFile("/proc/cpuinfo");
  auto cpuinfoLines = cpuinfo.split("\n\n").at(0).split("\n");

  foreach (auto line, cpuinfoLines) {
    auto splitLine = line.split(":");
    auto name = splitLine.at(0).trimmed();
    auto value = splitLine.at(1).trimmed();
    if (name == "vendor_id") {
      if (value == "GenuineIntel") {
        cpu->manufacturer = "Intel";
      } else if (value == "AuthenticAMD") {
        cpu->manufacturer = "AMD";
      } else {
        cpu->manufacturer = value;
      }
    } else if (name == "model name") {
      cpu->model = value;
    } else if (name == "siblings") {
      cpu->threads = value.toInt();
    } else if (name == "cpu cores") {
      cpu->cores = value.toInt();
    }
  }

  cpu->speed = getClockSpeed();
}

auto Device::setBios() -> void {
  bios = new Bios();
  bios->manufacturer = readDmi("bios_vendor");
  bios->version = readDmi("bios_version");
}

auto Device::setMainboard() -> void {
  mainboard = new Mainboard();
  mainboard->manufacturer = readDmi("board_vendor");
  mainboard->version = readDmi("board_version");
  mainboard->model = readDmi("board_name");
}

auto Device::setKernel() -> void {
  kernel = new Kernel();
  kernel->version = QSysInfo::kernelVersion();
  kernel->architecture = QSysInfo::currentCpuArchitecture();
}

auto Device::setDrives() -> void {
  std::vector<Drive *> d;
  QDir dir("/sys/class/block/");
  if (dir.isReadable() && dir.exists()) {
    dir.refresh();

    storage = 0;

    auto entryList = dir.entryList();

    foreach (auto entry, entryList) {
      if (entry == "." || entry == "..") {
        continue;
      }

      auto filePath = "/sys/class/block/" + entry;
      QDir info(filePath);
      if (!isPartition(info)) {
        auto drive = new Drive();
        drive->manufacturer = getDriveManufacturer(info);
        drive->model = getDriveModel(info);
        drive->name = info.dirName();
        drive->size = getDriveSize(info);
        d.emplace_back(drive);
        storage += drive->size;
      }
    }
  }

  drives = {d.begin(), d.end()};
}

auto Device::toJson() -> QJsonObject {
  QJsonObject json;
  json["id"] = id;
  json["hostname"] = hostname;
  json["model"] = model;
  json["manufacturer"] = manufacturer;
  json["storage"] = storage;
  json["ram"] = ram;

  json["os"] = os->toJson();
  json["cpu"] = cpu->toJson();
  json["bios"] = bios->toJson();
  json["mainboard"] = mainboard->toJson();
  json["kernel"] = kernel->toJson();

  QJsonArray jsonDrives;
  foreach (auto drive, drives) {
    jsonDrives.push_back(drive->toJson());
  }

  json["drives"] = jsonDrives;

  return json;
}
} // namespace jewels::device

#include "moc_device.cpp"