//
// Created by imanuel on 20.05.24.
//

#include "Jewels.h"
#include "device/device.h"
#include "jewelsconfig.h"
#include <QException>
#include <QFuture>
#include <QJsonDocument>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QNetworkRequestFactory>
#include <QtConcurrent>
#include <iostream>

namespace jewels {

auto Jewels::sendData() -> QPromise<void> * {
  auto promise = new QPromise<void>();
  promise->start();

  try {
    auto device = new jewels::device::Device();
    auto host = JewelsConfig::host();
    auto token = JewelsConfig::token();

    qInfo() << "Sending data to host " << host;
    QNetworkRequestFactory factory(host);
    factory.setBearerToken(token.toUtf8());

    qInfo() << "Sending data using token " << token;
    auto request = factory.createRequest("/api/device/computer");
    request.setHeader(QNetworkRequest::KnownHeaders::ContentTypeHeader,
                      "application/json");

    qInfo() << "Sending data to url " << request.url();

    QJsonDocument json(device->toJson());

    auto data = json.toJson(QJsonDocument::Compact);
    qInfo() << "Sending json body " << data;

    auto accessManager = new QNetworkAccessManager(this);

    auto reply = accessManager->post(request, data);
    connect(reply, &QNetworkReply::sslErrors, [promise](auto errors) {
      foreach (auto error, errors) {
        qWarning() << error.errorString();
      }
    });
    connect(reply, &QNetworkReply::finished, [promise]() {
      qInfo() << "Executed successfully";
      promise->finish();
    });
    connect(reply, &QNetworkReply::errorOccurred, [promise, reply](auto error) {
      PushDataException exception;

      qWarning() << "Failed to send network request" << error << ":"
                 << reply->errorString();
      if (QNetworkReply::ContentAccessDenied <= error &&
          error <= QNetworkReply::UnknownContentError) {
        auto data = reply->readAll();
        auto errorJson = QJsonDocument::fromJson(data);
        auto message = errorJson.object()["message"];

        qWarning() << message;
        exception.message = message.toString();
      }

      promise->setException(exception);
    });
  } catch (std::exception &exception) {
    PushDataException ex;
    ex.message = exception.what();
    promise->setException(ex);

    qWarning() << "Failed to collect data" << exception.what();
  }

  return promise;
}
} // namespace jewels