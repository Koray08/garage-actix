import React, { useState, useEffect } from "react";
import axios from "axios";
import { Link } from "react-router-dom";
import { Table, Button, Container, Card, Form, Row, Col } from "react-bootstrap";

const CarList = () => {
  const [cars, setCars] = useState([]);
  const [garages, setGarages] = useState([]);
  const [filters, setFilters] = useState({
    carMake: "",
    garageId: "",
    fromYear: "",
    toYear: "",
  });
  const [newCar, setNewCar] = useState({
    make: "",
    model: "",
    productionYear: "",
    licensePlate: "",
    garageIds: [],
  });
  const [isUpdating, setIsUpdating] = useState(false);
  const [carToUpdate, setCarToUpdate] = useState(null);
  const [recordToUpdate, setRecordToUpdate] = useState(null);


  useEffect(() => {
    fetchCars();
    fetchGarages();
  }, [filters]);

  const fetchCars = async () => {
    try {
      const response = await axios.get("http://localhost:8088/cars", {
        params: {
          carMake: filters.carMake || undefined,
          garageId: filters.garageId || undefined,
          fromYear: filters.fromYear || undefined,
          toYear: filters.toYear || undefined,
        },
      });
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

  const handleAddCar = async () => {
    try {
      const carData = {
        make: newCar.make,
        model: newCar.model,
        productionYear: parseInt(newCar.productionYear, 10),
        licensePlate: newCar.licensePlate,
        garageIds: newCar.garageIds.map(Number),
      };

      const response = await axios.post("http://localhost:8088/cars", carData);
      alert("Car added successfully!");

      setCars([...cars, response.data]);

      setNewCar({
        make: "",
        model: "",
        productionYear: "",
        licensePlate: "",
        garageIds: [],
      });
    } catch (error) {
      console.error("Error adding car:", error);
      alert("An error occurred while adding the car.");
    }
  };
  const handleUpdateCar = async () => {
    if (!carToUpdate || !carToUpdate.id) {
      alert("No car selected for update.");
      return;
    }
  
    try {
      console.log("Data being sent for updating:", carToUpdate);
      await axios.put(`http://localhost:8088/cars/${carToUpdate.id}`, carToUpdate);
      alert("Car updated successfully!");
  
      fetchCars(); // Обновете списъка с коли
      resetForm(); // Нулирайте формуляра
    } catch (error) {
      console.error("Error updating car:", error.response ? error.response.data : error);
      alert("An error occurred while updating the car.");
    }
  };
  
  
  const handleEditClick = (car) => {
    setRecordToUpdate({
      id: car.id, // ID на колата за обновяване
      make: car.make || "",
      model: car.model || "",
      year: car.year || "",
      licensePlate: car.licensePlate || "",
      garageIds: car.garages ? car.garages.map((garage) => garage.id) : [],
    });
    setIsUpdating(true); // Активира режим за обновяване
  };
  

  const handleCancelUpdate = () => {
    resetForm(); // Нулира формуляра
    setIsUpdating(false); // Връща режима на добавяне
  };
  

  const handleDeleteCar = async (carId) => {
    if (window.confirm("Are you sure you want to delete this car?")) {
      try {
        await axios.delete(`http://localhost:8088/cars/${carId}`);
        alert("Car deleted successfully!");
        fetchCars();
      } catch (error) {
        console.error("Error deleting car:", error);
      }
    }
  };

  const resetForm = () => {
    setNewCar({
      make: "",
      model: "",
      productionYear: "",
      licensePlate: "",
      garageIds: [],
    });
    setIsUpdating(false);
    setCarToUpdate(null);
  };

  return (
    <Container className="mt-4">
      <Card className="shadow-lg">
        <Card.Header style={{ backgroundColor: "#FFC107", color: "#000000" }}>
          <h3 className="text-center">Manage Cars</h3>
        </Card.Header>
        <Card.Body>
          <Form className="mb-3">
            <Row>
              <Col md={3}>
                <Form.Group>
                  <Form.Label>Filter by Make</Form.Label>
                  <Form.Control
                    type="text"
                    name="carMake"
                    placeholder="Enter make"
                    value={filters.carMake}
                    onChange={(e) =>
                      setFilters((prev) => ({ ...prev, carMake: e.target.value }))
                    }
                  />
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group>
                  <Form.Label>Filter by Garage</Form.Label>
                  <Form.Select
                    name="garageId"
                    value={filters.garageId}
                    onChange={(e) =>
                      setFilters((prev) => ({ ...prev, garageId: e.target.value }))
                    }
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
                  <Form.Label>From Year</Form.Label>
                  <Form.Control
                    type="number"
                    name="fromYear"
                    placeholder="Enter from year"
                    value={filters.fromYear}
                    onChange={(e) =>
                      setFilters((prev) => ({ ...prev, fromYear: e.target.value }))
                    }
                  />
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group>
                  <Form.Label>To Year</Form.Label>
                  <Form.Control
                    type="number"
                    name="toYear"
                    placeholder="Enter to year"
                    value={filters.toYear}
                    onChange={(e) =>
                      setFilters((prev) => ({ ...prev, toYear: e.target.value }))
                    }
                  />
                </Form.Group>
              </Col>
            </Row>
          </Form>

          <Table striped bordered hover responsive className="text-center">
            <thead style={{ backgroundColor: "#000000", color: "#FFC107" }}>
              <tr>
                <th>Make</th>
                <th>Model</th>
                <th>Year</th>
                <th>License Plate</th>
                <th>Garages</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {cars.map((car, index) => (
                <tr key={car.id}>
                  <td>{car.make}</td>
                  <td>{car.model}</td>
                  <td>{car.productionYear}</td>
                  <td>{car.licensePlate}</td>
                  <td>
                    {car.garages.map((garage) => garage.name).join(", ") ||
                      "Not Assigned"}
                  </td>
                  <td>
                    <Button
                      style={{
                        backgroundColor: "#FFC107",
                        border: "none",
                        color: "#000000",
                      }}
                      size="sm"
                      className="me-2"
                      onClick={() => {
                        setCarToUpdate(car);
                        setIsUpdating(true);
                      }}
                    >
                      Edit
                    </Button>
                    <Button
                      style={{
                        backgroundColor: "#000000",
                        border: "none",
                        color: "#FFC107",
                      }}
                      size="sm"
                      onClick={() => handleDeleteCar(car.id)}
                    >
                      Delete
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </Table>
          <Form>
            <Row>
              <Col md={3}>
                <Form.Group className="mb-3">
                  <Form.Label>Make</Form.Label>
                  <Form.Control
                    type="text"
                    placeholder="Enter make"
                    value={isUpdating ? carToUpdate?.make || "" : newCar.make}
                    onChange={(e) =>
                      isUpdating
                        ? setCarToUpdate((prev) => ({ ...prev, make: e.target.value }))
                        : setNewCar((prev) => ({ ...prev, make: e.target.value }))
                    }
                  />
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group className="mb-3">
                  <Form.Label>Model</Form.Label>
                  <Form.Control
                    type="text"
                    placeholder="Enter model"
                    value={isUpdating ? carToUpdate?.model || "" : newCar.model}
                    onChange={(e) =>
                      isUpdating
                        ? setCarToUpdate((prev) => ({ ...prev, model: e.target.value }))
                        : setNewCar((prev) => ({ ...prev, model: e.target.value }))
                    }
                  />
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group className="mb-3">
                  <Form.Label>Year</Form.Label>
                  <Form.Control
                    type="number"
                    placeholder="Enter year"
                    value={
                      isUpdating
                        ? carToUpdate?.productionYear || ""
                        : newCar.productionYear
                    }
                    onChange={(e) =>
                      isUpdating
                        ? setCarToUpdate((prev) => ({
                          ...prev,
                          productionYear: e.target.value,
                        }))
                        : setNewCar((prev) => ({
                          ...prev,
                          productionYear: e.target.value,
                        }))
                    }
                  />
                </Form.Group>
              </Col>
              <Col md={3}>
                <Form.Group className="mb-3">
                  <Form.Label>License Plate</Form.Label>
                  <Form.Control
                    type="text"
                    placeholder="Enter license plate"
                    value={
                      isUpdating
                        ? carToUpdate?.licensePlate || ""
                        : newCar.licensePlate
                    }
                    onChange={(e) =>
                      isUpdating
                        ? setCarToUpdate((prev) => ({
                          ...prev,
                          licensePlate: e.target.value,
                        }))
                        : setNewCar((prev) => ({
                          ...prev,
                          licensePlate: e.target.value,
                        }))
                    }
                  />
                </Form.Group>
              </Col>
            </Row>
            <Row>
              <Col md={12}>
                <Form.Group className="mb-3">
                  <Form.Label>Garages</Form.Label>
                  <Form.Select
                    multiple
                    value={isUpdating ? carToUpdate?.garageIds || [] : newCar.garageIds}
                    onChange={(e) => {
                      const selectedGarages = Array.from(
                        e.target.selectedOptions,
                        (option) => option.value
                      );
                      isUpdating
                        ? setCarToUpdate((prev) => ({
                          ...prev,
                          garageIds: selectedGarages,
                        }))
                        : setNewCar((prev) => ({
                          ...prev,
                          garageIds: selectedGarages,
                        }));
                    }}
                  >
                    {garages.map((garage) => (
                      <option key={garage.id} value={garage.id}>
                        {garage.name}
                      </option>
                    ))}
                  </Form.Select>
                </Form.Group>
              </Col>
            </Row>
            <div style={{ display: "flex", gap: "10px", marginTop: "10px" }}>
            <Button
  onClick={isUpdating ? handleUpdateCar : handleAddCar}
  style={{ backgroundColor: "#FFC107", border: "none", color: "#000000" }}
>
  {isUpdating ? "Update Car" : "Add Car"}
</Button>

{/* Бутон Cancel, показва се само при режим на обновяване */}
{isUpdating && (
  <Button
    onClick={handleCancelUpdate}
    style={{ backgroundColor: "#6c757d", border: "none", color: "#ffffff" }}
  >
    Cancel
  </Button>
 )}
</div>
          </Form>
        </Card.Body>
      </Card>
    </Container>
  );
};

export default CarList;



