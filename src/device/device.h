//
// Created by imanuel on 20.05.24.
//

#pragma once

#include <QDir>
#include <QFile>
#include <QJsonObject>
#include <QList>
#include <QObject>
#include <QQmlEngine>
#include <QQmlListProperty>
#include <QString>
#include <Qt>

namespace jewels::device {

using namespace Qt::StringLiterals;

const float ONE_DRIVE_GIB = 1'953'125.0f;
const float ONE_GIB = 1024.0f * 1024.0f * 1024.0f;

auto readFile(const QString &path) -> QString;

auto readDmi(const QString &dmi) -> QString;

class Drive : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(QString name MEMBER name NOTIFY nameChanged)
  Q_PROPERTY(
      QString manufacturer MEMBER manufacturer NOTIFY manufacturerChanged)
  Q_PROPERTY(QString model MEMBER model NOTIFY modelChanged)
  Q_PROPERTY(int size MEMBER size NOTIFY sizeChanged)

  QString name;
  QString manufacturer;
  QString model;
  float size;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void nameChanged();
  void manufacturerChanged();
  void modelChanged();
  void sizeChanged();
};

class Cpu : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(
      QString manufacturer MEMBER manufacturer NOTIFY manufacturerChanged)
  Q_PROPERTY(QString model MEMBER model NOTIFY modelChanged)
  Q_PROPERTY(int speed MEMBER speed NOTIFY speedChanged)
  Q_PROPERTY(int cores MEMBER cores NOTIFY coresChanged)
  Q_PROPERTY(int threads MEMBER threads NOTIFY threadsChanged)

  QString manufacturer;
  QString model;
  float speed;
  int cores;
  int threads;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void manufacturerChanged();
  void modelChanged();
  void speedChanged();
  void coresChanged();
  void threadsChanged();
};

class Bios : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(
      QString manufacturer MEMBER manufacturer NOTIFY manufacturerChanged)
  Q_PROPERTY(QString version MEMBER version NOTIFY versionChanged)

  QString manufacturer;
  QString version;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void manufacturerChanged();
  void versionChanged();
};

class Mainboard : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(
      QString manufacturer MEMBER manufacturer NOTIFY manufacturerChanged)
  Q_PROPERTY(QString version MEMBER version NOTIFY versionChanged)
  Q_PROPERTY(QString model MEMBER model NOTIFY modelChanged)

  QString manufacturer;
  QString version;
  QString model;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void manufacturerChanged();
  void versionChanged();
  void modelChanged();
};

class Kernel : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(QString version MEMBER version NOTIFY versionChanged)
  Q_PROPERTY(
      QString architecture MEMBER architecture NOTIFY architectureChanged)

  QString version;
  QString architecture;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void versionChanged();
  void architectureChanged();
};

class OperatingSystem : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  Q_PROPERTY(QString version MEMBER version NOTIFY versionChanged)
  Q_PROPERTY(QString name MEMBER name NOTIFY nameChanged)

  QString version;
  QString name;

  [[nodiscard]] auto toJson() const -> QJsonObject;

signals:
  void versionChanged();
  void nameChanged();
};

class Device : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  explicit Device() : Device(nullptr) {}

  explicit Device(QObject *parent);

  Q_PROPERTY(QString id MEMBER id NOTIFY idChanged)
  Q_PROPERTY(QString hostname MEMBER hostname NOTIFY hostnameChanged)
  Q_PROPERTY(QString model MEMBER model NOTIFY modelChanged)
  Q_PROPERTY(
      QString manufacturer MEMBER manufacturer NOTIFY manufacturerChanged)
  Q_PROPERTY(OperatingSystem *os MEMBER os NOTIFY osChanged)
  Q_PROPERTY(int storage MEMBER storage NOTIFY storageChanged)
  Q_PROPERTY(int ram MEMBER ram NOTIFY ramChanged)
  Q_PROPERTY(Cpu *cpu MEMBER cpu NOTIFY cpuChanged)
  Q_PROPERTY(Bios *bios MEMBER bios NOTIFY biosChanged)
  Q_PROPERTY(Mainboard *mainboard MEMBER mainboard NOTIFY mainboardChanged)
  Q_PROPERTY(Kernel *kernel MEMBER kernel NOTIFY kernelChanged)
  Q_PROPERTY(QList<Drive *> drives MEMBER drives NOTIFY drivesChanged)

  QString id;
  QString hostname;
  QString model;
  QString manufacturer;
  float storage;
  float ram;
  OperatingSystem *os;
  Cpu *cpu;
  Bios *bios;
  Mainboard *mainboard;
  Kernel *kernel;
  QList<Drive *> drives;

  auto toJson() -> QJsonObject;

signals:
  void idChanged();
  void hostnameChanged();
  void modelChanged();
  void manufacturerChanged();
  void osChanged();
  void storageChanged();
  void ramChanged();
  void cpuChanged();
  void biosChanged();
  void mainboardChanged();
  void kernelChanged();
  void drivesChanged();

private:
  auto setId() -> void;
  auto setHostname() -> void;
  auto setModel() -> void;
  auto setManufacturer() -> void;
  auto setOs() -> void;
  auto setRam() -> void;
  auto setCpu() -> void;
  auto setBios() -> void;
  auto setMainboard() -> void;
  auto setKernel() -> void;
  auto setDrives() -> void;
};
} // namespace jewels::device
