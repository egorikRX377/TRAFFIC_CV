import React, { useMemo, useState, useEffect } from "react";
import DataTable from "react-data-table-component";
import "bootstrap/dist/css/bootstrap.min.css";
import "@fortawesome/fontawesome-free/css/all.min.css";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";

const Dashboard = () => {
  const [search, setSearch] = useState("");
  const [telemetryData, setTelemetryData] = useState([]);
  const [lastUpdate, setLastUpdate] = useState(new Date());

  // ====================== Запрос телеметрии ======================
  const fetchTelemetry = async () => {
    try {
      const res = await fetch("http://localhost:8080/operator/telemetry");
      if (!res.ok) throw new Error("Ошибка при получении телеметрии");
      const data = await res.json();
      setTelemetryData(data);
      setLastUpdate(new Date());
    } catch (err) {
      console.error("Ошибка при получении телеметрии:", err);
    }
  };

  useEffect(() => {
    fetchTelemetry();
    const intervalId = setInterval(fetchTelemetry, 20000);
    return () => clearInterval(intervalId);
  }, []);

  // ====================== Колонки таблицы ======================
  const columns = [
    { name: "Устройство", selector: row => row.device_name, sortable: true },
    { name: "IP", selector: row => row.ip_address, sortable: true },
    { name: "Локация", selector: row => row.location, sortable: true },
    {
      name: "Статус",
      selector: row => row.metric_value,
      sortable: true,
      cell: row => (
        <span className={`badge bg-${row.metric_value > 80 ? "warning" : "success"}`}>
          {row.metric_value > 80 ? "warning" : "active"}
        </span>
      ),
    },
    { name: "Метрика", selector: row => row.action_description || "Загрузка", sortable: true },
    { name: "Значение", selector: row => row.metric_value, sortable: true },
    {
      name: "Аномалия",
      selector: row => row.metric_value,
      sortable: true,
      cell: row => (row.metric_value > 90 ? "Да" : "—"),
    },
  ];

  // ====================== Поиск ======================
  const filteredData = useMemo(() => {
    return telemetryData.filter(row =>
      Object.values(row).join(" ").toLowerCase().includes(search.toLowerCase())
    );
  }, [search, telemetryData]);

  // ====================== ФОРМИРОВАНИЕ ДАННЫХ ДЛЯ ГРАФИКОВ ======================

  const formatTime = (dateString) => {
    return new Date(dateString).toLocaleTimeString("ru-RU", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
  };

  // 1. Статус устройств — среднее
  const statusChartData = useMemo(() => {
    const byTime = {};
    telemetryData.forEach(item => {
      const time = formatTime(item.recorded_at);
      if (!byTime[time]) byTime[time] = { time, values: [] };
      byTime[time].values.push(item.metric_value);
    });
    return Object.values(byTime)
      .map(slot => ({
        time: slot.time,
        value: Math.round(slot.values.reduce((a, b) => a + b, 0) / slot.values.length),
      }))
      .slice(-50);
  }, [telemetryData]);

  // 2. Тренды аномалий
  const anomaliesChartData = useMemo(() => {
    const byTime = {};
    telemetryData.forEach(item => {
      const time = formatTime(item.recorded_at);
      if (!byTime[time]) byTime[time] = { time, count: 0 };
      if (item.metric_value > 90) byTime[time].count += 1;
    });
    return Object.values(byTime)
      .map(slot => ({ time: slot.time, value: slot.count }))
      .slice(-50);
  }, [telemetryData]);

  // 3. Производительность сети — максимум
  const performanceChartData = useMemo(() => {
    const byTime = {};
    telemetryData.forEach(item => {
      const time = formatTime(item.recorded_at);
      if (!byTime[time]) byTime[time] = { time, max: 0 };
      if (item.metric_value > byTime[time].max) {
        byTime[time].max = item.metric_value;
      }
    });
    return Object.values(byTime)
      .map(slot => ({ time: slot.time, value: slot.max }))
      .slice(-50);
  }, [telemetryData]);

  return (
    <div className="sb-nav-fixed">
      {/* Топбар */}
      <nav className="sb-topnav navbar navbar-dark bg-dark">
        <a className="navbar-brand ps-3" href="#!">Мониторинг сети</a>
      </nav>

      {/* Основной контент — без боковой панели */}
      <div className="p-4">
        <h1 className="mb-4">Телеметрия в реальном времени</h1>

        {/* Таблица */}
        <div className="card mb-4">
          <div className="card-header d-flex justify-content-between">
            <span>Текущие данные</span>
            <small className="text-muted">Обновлено: {lastUpdate.toLocaleTimeString()}</small>
          </div>
          <div className="card-body">
            <input
              type="text"
              className="form-control mb-3"
              placeholder="Поиск (Router, IP, локация...)"
              value={search}
              onChange={(e) => setSearch(e.value)}
            />
            <DataTable
              columns={columns}
              data={filteredData}
              pagination
              striped
              highlightOnHover
              dense
              customStyles={{
                rows: {
                  style: (row) => ({
                    backgroundColor: row.metric_value > 90 ? "#ffdddd" : "white",
                  }),
                },
              }}
            />
          </div>
        </div>

        {/* Три графика */}
        <div className="row">
          {/* 1. Статус устройств */}
          <div className="col-xl-4 mb-4">
            <div className="card h-100">
              <div className="card-header">Статус устройств (среднее)</div>
              <div className="card-body" style={{ height: 300 }}>
                {statusChartData.length > 0 ? (
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={statusChartData}>
                      <CartesianGrid strokeDasharray="3 3" />
                      <XAxis dataKey="time" angle={-45} textAnchor="end" height={60} />
                      <YAxis domain={[0, 100]} />
                      <Tooltip />
                      <Legend />
                      <Line type="monotone" dataKey="value" stroke="#28a745" name="Среднее %" strokeWidth={2} dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                ) : (
                  <p className="text-muted text-center mt-5">Нет данных</p>
                )}
              </div>
            </div>
          </div>

          {/* 2. Тренды аномалий */}
          <div className="col-xl-4 mb-4">
            <div className="card h-100">
              <div className="card-header">Тренды аномалий (кол-во больше 90)</div>
              <div className="card-body" style={{ height: 300 }}>
                {anomaliesChartData.length > 0 ? (
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={anomaliesChartData}>
                      <CartesianGrid strokeDasharray="3 3" />
                      <XAxis dataKey="time" angle={-45} textAnchor="end" height={60} />
                      <YAxis />
                      <Tooltip />
                      <Legend />
                      <Line type="monotone" dataKey="value" stroke="#dc3545" name="Аномалий" strokeWidth={2} dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                ) : (
                  <p className="text-muted text-center mt-5">Нет данных</p>
                )}
              </div>
            </div>
          </div>

          {/* 3. Производительность сети */}
          <div className="col-xl-4 mb-4">
            <div className="card h-100">
              <div className="card-header">Производительность сети (макс.)</div>
              <div className="card-body" style={{ height: 300 }}>
                {performanceChartData.length > 0 ? (
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={performanceChartData}>
                      <CartesianGrid strokeDasharray="3 3" />
                      <XAxis dataKey="time" angle={-45} textAnchor="end" height={60} />
                      <YAxis domain={[0, 100]} />
                      <Tooltip />
                      <Legend />
                      <Line type="monotone" dataKey="value" stroke="#007bff" name="Максимум %" strokeWidth={2} dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                ) : (
                  <p className="text-muted text-center mt-5">Нет данных</p>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Футер */}
      <footer className="py-3 bg-light mt-auto text-center">
        <small className="text-muted">© Система мониторинга сети, 2025</small>
      </footer>
    </div>
  );
};

export default Dashboard;