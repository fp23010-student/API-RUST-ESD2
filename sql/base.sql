CREATE TABLE Carreras (
    id_carrera SERIAL PRIMARY KEY,
    nombre_carrera VARCHAR(100) NOT NULL,
    facultad VARCHAR(100) NOT NULL
);

CREATE TABLE Estudiantes (
    id_estudiante SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    carnet VARCHAR(10) UNIQUE NOT NULL,
    fecha_nacimiento DATE,
    id_carrera INT REFERENCES Carreras(id_carrera)
);

CREATE TABLE profesores (
    id_profesor SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    correo VARCHAR(150) UNIQUE NOT NULL,
    especialidad VARCHAR(100) NOT NULL
);

CREATE TABLE Materias (
    id_materia SERIAL PRIMARY KEY,
    nombre_materia VARCHAR(100) NOT NULL,
    uv INT CHECK (uv > 0),
    id_profesor INT REFERENCES Profesores(id_profesor)
);

CREATE TABLE Inscripciones (
    id_inscripcion SERIAL PRIMARY KEY,
    id_estudiante INT REFERENCES Estudiantes(id_estudiante),
    id_materia INT REFERENCES Materias(id_materia),
    ciclo VARCHAR(10) NOT NULL,
    nota_final DECIMAL(4,2) CHECK (nota_final BETWEEN 0 AND 10)
);
