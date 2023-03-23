#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    HIGHIMP,
    UNKNOWN,
    ZERO,
    ONE
}

pub struct Process {
    inputs: Vec<usize>,
    output: Value
}

impl Process {
    fn with_output(value: Value) -> Self {
        Process {
            inputs: vec![],
            output: value
        }
    }

    fn new() -> Self {
        Process::with_output(Value::HIGHIMP)
    }
}

pub struct Net {
    driver: Option<usize>
}

impl Net {
    fn from_index(driver: usize) -> Self {
        Net {
            driver: Some(driver)
        }
    }

    fn new() -> Self {
        Net {
            driver: None
        }
    }
}

pub struct State {
    processes: Vec<Process>,
    nets: Vec<Net>
}

impl State {
    pub fn new() -> Self {
        State {
            processes: vec![],
            nets: vec![]
        }
    }

    pub fn set_process_output(&mut self, index: usize, value: Value) {
        if let Some(process) = self.processes.get_mut(index) {
            process.output = value;
        }
    }

    fn find_net_driver_value(&self, index: usize) -> Value {
        if let Some(net) = self.nets.get(index) {
            if let Some(driver) = net.driver {
                if let Some(value) = self.get_procedure_output(driver) {
                    return value;
                }
            }
        }
        Value::HIGHIMP
    }
 
    fn get_procedure_output(&self, index: usize) -> Option<Value> {
        if let Some(process) = self.processes.get(index) {
            return Some(process.output);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_net_driver_value_some() {
        let mut state = State::new();

        state.processes.push(Process::with_output(Value::ONE));
        state.processes.push(Process::with_output(Value::ZERO));
        state.processes.push(Process::with_output(Value::UNKNOWN));

        state.nets.push(Net::from_index(0));
        state.nets.push(Net::from_index(1));
        state.nets.push(Net::from_index(2));

        assert_eq!(state.find_net_driver_value(0), Value::ONE);
        assert_eq!(state.find_net_driver_value(1), Value::ZERO);
        assert_eq!(state.find_net_driver_value(2), Value::UNKNOWN);
    }

    #[test]
    fn find_net_driver_value_no_driver() {
        let mut state = State::new();

        state.processes.push(Process::new());
        state.nets.push(Net::new());

        assert_eq!(state.find_net_driver_value(0), Value::HIGHIMP);
    }

    #[test]
    fn find_net_driver_value_process_dne() {
        let mut state = State::new();

        state.nets.push(Net::from_index(0));

        assert_eq!(state.find_net_driver_value(0), Value::HIGHIMP);
    }


    #[test]
    fn find_net_driver_value_net_dne() {
        let state = State::new();

        assert_eq!(state.find_net_driver_value(0), Value::HIGHIMP);
    }

    #[test]
    fn set_process_output_some() {
        let mut state = State::new();
        state.processes.push(Process::new());
        state.set_process_output(0, Value::ONE);

        assert_eq!(state.processes[0].output, Value::ONE);
    }
}