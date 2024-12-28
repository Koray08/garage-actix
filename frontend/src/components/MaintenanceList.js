import React, { useState, useEffect } from "react";
import axios from "axios";
import { Table, Button, Container, Card, Form, Row, Col } from "react-bootstrap";
import ReactDatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";

const MaintenanceList = () => {
  const [maintenances, setMaintenances] = useState([]);
  const [cars, setCars] = useState([]);
  const [garages, setGarages] = useState([]);
  const [monthlyReportData, setMonthlyReportData] = useState([]);
  const [filterGarageId, setFilterGarageId] = useState("");
  const [filterCarId, setFilterCarId] = useState("");
  const [filterStartDate, setFilterStartDate] = useState("");
  const [filterEndDate, setFilterEndDate] = useState("");
  const [reportFilter, setReportFilter] = useState({ garageId: "",  scheduledDate: "", });
  const [maintenanceFormData, setMaintenanceFormData] = useState({
    type: "",
    date: "",
    car: "",
    garage: "",
  });
  

  const [monthlyReportFilter, setMonthlyReportFilter] = useState({
    garageId: "",
    startMonth: "",
    endMonth: "",
  });
  const [newMaintenance, setNewMaintenance] = useState({
    serviceType: "",
    scheduledDate: "",
    garageId: "",
    carId: "",
  });
  const [isUpdating, setIsUpdating] = useState(false);
  const [maintenanceToUpdate, setMaintenanceToUpdate] = useState(null);
  const [newRecord, setNewRecord] = useState({
    serviceType: "",
    scheduledDate: "",
    garageId: "",
    carId: "",
  });
  const [startMonth, setStartMonth] = useState(new Date());
  const [endMonth, setEndMonth] = useState(new Date());
  const [recordToUpdate, setRecordToUpdate] = useState({
    id: "",
    serviceType: "",
    scheduledDate: "",
    garageId: "",
    carId: "",
  });
  const [filterState, setFilterState] = useState({
    carId: "",
    garageId: "",
    startDate: "",
    endDate: "",
  });
  

  const fetchMaintenances = async () => {
    try {
      const response = await axios.get("http://localhost:8088/maintenance", {
        params: {
          carId: filterCarId || undefined,
          garageId: filterGarageId || undefined,
          scheduledDate: reportFilter.scheduledDate || undefined, // Една дата вместо диапазон
        },
      });
  
      const [garagesResponse, carsResponse] = await Promise.all([
        axios.get("http://localhost:8088/garages"),
        axios.get("http://localhost:8088/cars"),
      ]);
  
      const garages = garagesResponse.data;
      const cars = carsResponse.data;
  
      const enrichedMaintenances = response.data.map((maintenance) => {
        const garage = garages.find((g) => g.id === maintenance.garageId) || {};
        const car = cars.find((c) => c.id === maintenance.carId) || {};
        return {
          ...maintenance,
          garageName: garage.name || "N/A",
          carName: car.make ? `${car.make} ${car.model}` : "N/A",
        };
      });
  
      setMaintenances(enrichedMaintenances);
    } catch (error) {
      console.error("Error fetching maintenances:", error);
    }
  };
  

  const fetchCars = async () => {
    try {
      const response = await axios.get("http://localhost:8088/cars");
      setCars(response.data);
    } catch (error) {
      console.error("Error fetching cars:", error);
    }
  };
  const fetchGarages = async () => {
    try {
      const response = await axios.get("http://localhost:8088/garages");
      setGarages(response.data);
    } catch (error) {
      console.error("Error fetching garages:", error);
    }
  };


  const handleReportFilterChange = (e) => {
    const { name, value } = e.target;
    setReportFilter((prev) => ({ ...prev, [name]: value }));
  };

  const handleAddMaintenance = async () => {
    try {
      const response = await axios.post("http://localhost:8088/maintenance", newRecord);
      alert("Record added successfully!");
    } catch (error) {
      console.error("Error adding record:", error);
    }
  };
  const handleChange = (e) => {
    const { name, value } = e.target;
    setNewMaintenance((prev) => ({
      ...prev,
      [name]: value || "", // Никога не оставяй `undefined`
    }));
  };
  
  
  const handleAddRecord = async () => {
    try {
      // Проверете дали всички полета са попълнени
      if (!newMaintenance.serviceType || !newMaintenance.scheduledDate || !newMaintenance.garageId || !newMaintenance.carId) {
        alert("Please fill all fields.");
        return;
      }
  
      // Намерете избрания гараж
      const selectedGarage = garages.find((garage) => garage.id === parseInt(newMaintenance.garageId, 10));
      if (!selectedGarage) {
        alert("Selected garage is not valid.");
        return;
      }
  
      // Пребройте текущия брой поддръжки в този гараж
      const currentMaintenanceCount = maintenances.filter(
        (maintenance) => maintenance.garageId === parseInt(newMaintenance.garageId, 10)
      ).length;
  
      // Проверете дали капацитетът ще бъде надвишен
      if (currentMaintenanceCount >= selectedGarage.capacity) {
        alert(`Cannot add maintenance. Garage capacity (${selectedGarage.capacity}) reached.`);
        return;
      }
  
      console.log("Data being sent to the server:", newMaintenance); // Логнете данните
  
      // Изпращане на заявката към API
      const response = await axios.post("http://localhost:8088/maintenance", newMaintenance);
      alert("Record added successfully!");
  
      // Обновете таблицата
      setMaintenances((prev) => [...prev, response.data]);
  
      // Нулирайте формата
      setNewMaintenance({
        serviceType: "",
        scheduledDate: "",
        garageId: "",
        carId: "",
      });
    } catch (error) {
      console.error("Error adding record:", error.response ? error.response.data : error);
      alert("An error occurred while adding the record.");
    }
  };
  
  const handleCancelUpdate = () => {
    resetForm(); // Нулира формуляра за поддръжка
    setIsUpdating(false); // Връща режима към "Добавяне"
  };
  

  const handleUpdateRecord = async () => {
    if (!recordToUpdate || !recordToUpdate.id) {
      alert("No maintenance record selected for update.");
      return;
    }
  
    try {
      console.log("Updating record with data:", recordToUpdate); // Лог за проверка
      await axios.put(`http://localhost:8088/maintenance/${recordToUpdate.id}`, recordToUpdate);
      alert("Maintenance record updated successfully!");
  
     
    resetForm();

    setIsUpdating(false);
  } catch (error) {
    console.error("Error updating maintenance record:", error);
    alert("An error occurred while updating the record.");
  }
  };
  
  const resetForm = () => {
    setMaintenanceFormData({
      type: "",
      date: "",
      car: "",
      garage: "",
    });
  };
  
  
  
  const handleEditClick = (maintenance) => {
    console.log("Selected maintenance for editing:", maintenance); // Логнете данните за проверка
  
    setRecordToUpdate({
      id: maintenance.id,
      serviceType: maintenance.serviceType || "",
      scheduledDate: maintenance.scheduledDate || "",
      garageId: maintenance.garageId || "",
      carId: maintenance.carId || "",
    });
  
    setIsUpdating(true);
  };
  
  


  const handleDeleteMaintenance = async (maintenanceId) => {
    if (window.confirm("Are you sure you want to delete this maintenance request?")) {
      try {
        await axios.delete(`http://localhost:8088/maintenance/${maintenanceId}`);
        alert("Maintenance request deleted successfully!");
        fetchMaintenances();
      } catch (error) {
        console.error("Error deleting maintenance:", error);
        alert("An error occurred while deleting the maintenance request.");
      }
    }
  };

  const filteredMaintenances = maintenances.filter((maintenance) => {
    const { carId, garageId, startDate, endDate } = filterState;
  
    if (carId && maintenance.carId !== parseInt(carId)) return false;
  
    if (garageId && maintenance.garageId !== parseInt(garageId)) return false;
  
    if (startDate && new Date(maintenance.scheduledDate) < new Date(startDate)) return false;
  
    if (endDate && new Date(maintenance.scheduledDate) > new Date(endDate)) return false;
  
    return true; 
  });
  

  const fetchMonthlyReport = async () => {
    const { garageId, startMonth, endMonth } = monthlyReportFilter;

    if (!garageId || !startMonth || !endMonth) {
      alert("Please select all fields.");
      return;
    }

    console.log("Sending request with filter:", { garageId, startMonth, endMonth });

    try {
      const response = await axios.get("http://localhost:8088/maintenance/monthlyRequestsReport", {
        params: { garageId, startMonth, endMonth },
      });

      if (response.data && response.data.length > 0) {
        for (let resp of response.data) {
          if (typeof resp.yearMonth.year!="undefined") {
            let yearMonth = resp.yearMonth.year + "-" + String(resp.yearMonth.monthValue).padStart(2, "0");
            resp.yearMonth = yearMonth;
          }
        }
        setMonthlyReportData(response.data);
      } else {
        setMonthlyReportData([]);
        alert("No data available for the selected criteria.");
      }
    } catch (error) {
      console.error("Error fetching monthly report:", error);
    }
  };

  const handleGarageChange = (e) => {
    setMonthlyReportFilter((prev) => ({
      ...prev,
      garageId: e.target.value,
    }));
  };


  useEffect(() => {
    fetchCars();
    fetchGarages();
  }, []);

  useEffect(() => {
    fetchMaintenances();
  }, [filterCarId, filterGarageId, filterStartDate, filterEndDate, reportFilter]);

  return (
    <Container className="mt-4">
      <Card className="shadow-lg">
        <Card.Header style={{ backgroundColor: "#FFC107", color: "#000000" }}>
          <h3 className="text-center">Manage Maintenance</h3>
        </Card.Header>
        <Card.Body>
          <Form className="mb-3">
            <Row>
              <Col md={3}>
                <Form.Group>
                  <Form.Label>Filter by Car</Form.Label>
                  <Form.Select
                    value={filterCarId}
                    onChange={(e) => setFilterCarId(e.target.value)}
                  >
                    <option value="">All Cars</option>
                    {cars.map((car) => (
                      <option key={car.id} value={car.id}>
                        {car.make} {car.model}
                      </option>
                    ))}
                  </Form.Select>
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group>
                  <Form.Label>Filter by Garage</Form.Label>
                  <Form.Select
                    value={filterGarageId}
                    onChange={(e) => setFilterGarageId(e.target.value)}
                  >
                    <option value="">All Garages</option>
                    {garages.map((garage) => (
                      <option key={garage.id} value={garage.id}>
                        {garage.name}
                      </option>
                    ))}
                  </Form.Select>
                </Form.Group>
              </Col>
              <Col md={3}>
              <Form.Group>
                  <Form.Label>Start Date</Form.Label>
                  <Form.Control
  type="date"
  value={filterState.startDate || ""}
  onChange={(e) => setFilterState({ ...filterState, startDate: e.target.value })}
/> 
</Form.Group>
</Col>
<Col md={3}>           
<Form.Group>
<Form.Label>End Date</Form.Label>
<Form.Control
  type="date"
  value={filterState.endDate || ""}
  onChange={(e) => setFilterState({ ...filterState, endDate: e.target.value })}
/>
                </Form.Group>
              </Col>
            </Row>
          </Form>
          <Table striped bordered hover responsive className="text-center">
            <thead style={{ backgroundColor: "#000000", color: "#FFC107" }}>
              <tr>
                <th>Service Type</th>
                <th>Date</th>
                <th>Garage</th>
                <th>Car</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredMaintenances.map((maintenance, index) => (
                <tr key={maintenance.id}>
                  <td>{maintenance.serviceType}</td>
                  <td>{maintenance.scheduledDate}</td>
                  <td>{maintenance.garageName}</td>
                  <td>{maintenance.carName}</td>
                  <td>
                  <Button
  style={{ backgroundColor: "#FFC107", border: "none", color: "#000000" }}
  size="sm"
  className="me-2"
  onClick={() => handleEditClick(maintenance)}
>
                      Edit
                    </Button>
                    <Button
                      style={{ backgroundColor: "#000000", border: "none", color: "#FFC107" }}
                      size="sm"
                      onClick={() => handleDeleteMaintenance(maintenance.id)}
                    >
                      Delete
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </Table>
          <h4 className="mt-4">{isUpdating ? "Update Maintenance Record" : "Add a New Record"}</h4>
          <Form>
            <Row>
              <Col md={4}>






      <Form.Group className="mb-3">
        <Form.Label>Type</Form.Label>
        
        <Form.Control
  type="text"
  name="serviceType"
  value={isUpdating ? recordToUpdate?.serviceType || "" : newMaintenance.serviceType || ""}
  onChange={(e) =>
    isUpdating
      ? setRecordToUpdate((prev) => ({ ...prev, serviceType: e.target.value }))
      : setNewMaintenance((prev) => ({ ...prev, serviceType: e.target.value }))
  }
/>
      </Form.Group>
    </Col>
    <Col md={4}>
      <Form.Group className="mb-3">
        <Form.Label>Date</Form.Label>
        <Form.Control
          type="date"
          name="scheduledDate"
          value={isUpdating ? recordToUpdate?.scheduledDate || "" : newMaintenance.scheduledDate}
          onChange={(e) =>
            isUpdating
              ? setRecordToUpdate((prev) => ({ ...prev, scheduledDate: e.target.value }))
              : setNewMaintenance((prev) => ({ ...prev, scheduledDate: e.target.value }))
          }
        />
      </Form.Group>
    </Col>
    <Col md={4}>
      <Form.Group className="mb-3">
        <Form.Label>Garage</Form.Label>
        <Form.Select
  name="garageId"
  value={isUpdating ? recordToUpdate?.garageId || "" : newMaintenance.garageId || ""}
  onChange={(e) =>
    isUpdating
      ? setRecordToUpdate((prev) => ({ ...prev, garageId: e.target.value }))
      : setNewMaintenance((prev) => ({ ...prev, garageId: e.target.value }))
  }
>
  <option value="">Select a Garage</option>
  {garages.map((garage) => (
    <option key={garage.id} value={garage.id}>
      {garage.name}
    </option>
  ))}
</Form.Select>
      </Form.Group>
    </Col>
    <Col md={4}>
      <Form.Group className="mb-3">
        <Form.Label>Car</Form.Label>
        <Form.Select
          name="carId"
          value={isUpdating ? recordToUpdate?.carId || "" : newMaintenance.carId}
          onChange={(e) =>
            isUpdating
              ? setRecordToUpdate((prev) => ({ ...prev, carId: e.target.value }))
              : setNewMaintenance((prev) => ({ ...prev, carId: e.target.value }))
          }
        >
          <option value="">Select a Car</option>
          {cars.map((car) => (
            <option key={car.id} value={car.id}>
              {car.make} {car.model}
            </option>
          ))}
        </Form.Select>
      </Form.Group>
    </Col>
  </Row>

  <div style={{ display: "flex", gap: "10px", marginTop: "10px" }}>
  <Button
            style={{
              backgroundColor: "#FFC107",
              border: "none",
              color: "#000000",
            }}
            onClick={isUpdating ? handleUpdateRecord : handleAddRecord}
          >
            {isUpdating ? "Update Record" : "Add Record"}
          </Button>
          {isUpdating && (
            <Button
              style={{
                backgroundColor: "#6c757d",
                border: "none",
                color: "#ffffff",
              }}
              onClick={handleCancelUpdate}
            >
              Cancel
            </Button>
  )}
</div>


</Form>

          <h4 className="mt-4">Monthly Report</h4>
          <Form className="mb-3">
            <Row className="align-items-end">
              <Col md={4}>
                <Form.Group className="mb-3">
                  <Form.Label className>Garage</Form.Label>
                  <Form.Select
                    value={monthlyReportFilter.garageId}
                    onChange={handleGarageChange}
                  >
                    <option value="">Select a garage</option>
                    {garages.map((garage) => (
                      <option key={garage.id} value={garage.id}>
                        {garage.name}
                      </option>
                    ))}
                  </Form.Select>
                </Form.Group>
              </Col>
              <Col md={4}>
                <Form.Group className="mb-3">
                  <Form.Label>Start:</Form.Label>
                  <ReactDatePicker
                    selected={
                      monthlyReportFilter.startMonth && /^\d{4}-\d{2}$/.test(monthlyReportFilter.startMonth)
                        ? new Date(`${monthlyReportFilter.startMonth}-01`)
                        : null
                    }
                    onChange={(date) => {
                      if (date) {
                        const formattedDate = `${date.getFullYear()}-${(date.getMonth() + 1)
                          .toString()
                          .padStart(2, "0")}`;
                        setMonthlyReportFilter((prev) => ({ ...prev, startMonth: formattedDate }));
                      }
                    }}
                    dateFormat="yyyy-MM"
                    showMonthYearPicker
                    className="form-control"
                  />
                </Form.Group>
              </Col>
              <Col md={4}>
                <Form.Group className="mb-3">
                  <Form.Label >End:</Form.Label>
                  <ReactDatePicker
                    selected={
                      monthlyReportFilter.endMonth && /^\d{4}-\d{2}$/.test(monthlyReportFilter.endMonth)
                        ? new Date(`${monthlyReportFilter.endMonth}-01`)
                        : null
                    }
                    onChange={(date) => {
                      if (date) {
                        const formattedDate = `${date.getFullYear()}-${(date.getMonth() + 1)
                          .toString()
                          .padStart(2, "0")}`;
                        setMonthlyReportFilter((prev) => ({ ...prev, endMonth: formattedDate }));
                      }
                    }}
                    dateFormat="yyyy-MM"
                    showMonthYearPicker
                    className="form-control"
                  />
                </Form.Group>
              </Col>
            </Row>
            <Button
              style={{ backgroundColor: "#FFC107", border: "none", color: "#000000" }}
              className="w-100 mt-3"
              onClick={fetchMonthlyReport}
            >
              Generate Monthly Report
            </Button>
          </Form>
          <Table striped bordered hover responsive className="text-center mt-4">
            <thead style={{ backgroundColor: "#000000", color: "#FFC107" }}>
              <tr>
                <th>Month</th>
                <th>Requests</th>
              </tr>
            </thead>
            <tbody>
              {monthlyReportData.length > 0 ? (
                monthlyReportData.map((report, index) => (
                  <tr key={index}>
                    <td>{report.yearMonth || "N/A"}</td>
                    <td>{report.requests || 0}</td>
                  </tr>
                ))
              ) : (
                <tr>
                  <td colSpan="3">No data available for the selected criteria.</td>
                </tr>
              )}
            </tbody>
          </Table>
        </Card.Body>
      </Card>
    </Container>
  );
};

export default MaintenanceList;

