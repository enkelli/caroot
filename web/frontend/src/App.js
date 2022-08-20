import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { ColorForState } from './Colors';

import { FaGithub } from 'react-icons/fa';

import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import FormControl from "react-bootstrap/FormControl";
import InputGroup from "react-bootstrap/InputGroup";
import Row from "react-bootstrap/Row";


function CA(props) {
    if (props.steps.length == 0) {
        return (
            <Container fluid>
            </Container>
        );
    }

    const caRows = [];
    const height = Math.floor((0.5 * window.screen.height) / props.steps.length);
    const width = Math.floor((0.75 * window.screen.width) / props.steps[0].length);
    const sq_len = Math.min(width, height);
    const margin = sq_len < 15 ? 1 : 3;

    for (let row_index = 0; row_index < props.steps.length; row_index++) {
        const step = props.steps[row_index];
        const cells = [];
        console.log(step.length, window.screen.width, window.screen.height, width, height)
        for (let i = 0; i < step.length; i++ ) {
            cells.push(
                <Button
                    variant="default"
                    style={{ background: ColorForState(step[i]), height: `${sq_len}px`, width: `${sq_len}px`, marginRight: `${margin}px`, padding: 0 }}
                    key={ `cell-${i}` }
                    disabled
                ></Button>
            )
        }
        caRows.push(
            <Row
                key={ `row-${row_index}` }
                className="my-1"
                style={{ height: `${sq_len}px` }}
            >
                <Col>{cells}</Col>
            </Row>
        );
    }
    return (
        <Container fluid className="my-5">
            {caRows}
        </Container>
    );
}

function NumberForm(props) {

  function OnChangeHandler(e) {
    if (!e.target.checkValidity()) {
        e.target.reportValidity();
    } else {
        return props.onNumberChange(e);
    }
  }

  return (
    <Col md={{ span: 2, offset: 5 }}>
      <InputGroup className="mt-3">
        <InputGroup.Prepend>
          <InputGroup.Text id="number">√</InputGroup.Text>
        </InputGroup.Prepend>
        <FormControl
          aria-label="number"
          aria-describedby="number"
          type="number"
          min="1"
          max="100"
          value={props.number}
          onChange={OnChangeHandler}
        />
        <InputGroup.Prepend>
          <InputGroup.Text id="sqrt">= {props.sqrt}</InputGroup.Text>
        </InputGroup.Prepend>
      </InputGroup>
    </Col>
  );
}

function App() {
  const [caSteps, setCaSteps] = useState([]);
  const [number, setNumber] = useState(25);
  const [sqrt, setSqrt] = useState(null);
  const [timer, setTimer] = useState(null);

  function updateSteps() {
    fetch(`/api/sqrt/${number}/steps`)
      .then(function(res) {
        if (!res.ok) {
            throw Error(res.statusText);
        }
        return res.json();
      })
      .then((res) => {
        setCaSteps(res.steps);
        setSqrt(res.sqrt);
    }).catch((err) => console.log(err));
  }

  function handleNumberUpdate(e) {
    setNumber(e.target.value);
  }
  useEffect(() => {
    if (timer) {
      clearTimeout(timer);
      setTimer(null);
    }
    setTimer(
      setTimeout(() => {
        updateSteps();
      }, 300)
    );
  }, [number]);

  return (
    <div className="App">
      <header className="App-header">
        <div className="my-1">
            <p>
                CAROOT
                <img className = "img img-fluid ml-2"
                    style={{ width: 50, height: 50 }}
                    src={ logo }
                />
            </p>
            <p>
                Cellular Automaton for square ROOT
                &nbsp;
                <a href="https://github.com/enkelli/caroot" target="_blank" style={{ color: "inherit" }}><FaGithub /></a>
            </p>
        </div>
      </header>
      <Container className="justify-content-md-center">
        <Row>
          <NumberForm
            number={number}
            sqrt={sqrt}
            onNumberChange={(e) => handleNumberUpdate(e)}
          />
        </Row>
      </Container>
      <CA steps={caSteps}/>
    </div>
  );
}

export default App;
