#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    HIGHIMP,
    UNKNOWN,
    ZERO,
    ONE
}

pub struct Process {
    inputs: Vec<Option<usize>>,
    output: Option<usize>,
    output_value: Value 
}

impl Process {
    fn with_output(value: Value) -> Self {
        Process {
            inputs: vec![],
            output: None,
            output_value: value
        }
    }

    fn new() -> Self {
        Process::with_output(Value::HIGHIMP)
    } 
}

pub struct Net {
    driver: Option<usize>,
    sensitive_processes: Vec<usize>
}

impl Net {
    fn new() -> Self {
        Net {
            driver: None,
            sensitive_processes: vec![]
        }
    }

    fn add_sensitive_process(&mut self, proc_index: usize) {
        self.sensitive_processes.push(proc_index);
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

    pub fn add_process(&mut self) -> usize {
        self.processes.push(Process::new());
        self.processes.len() - 1
    }
    
    pub fn add_net(&mut self) -> usize {
        self.nets.push(Net::new());
        self.nets.len() - 1
    }
    
    pub fn set_process_inputs(&mut self, process_index: usize, input_nets: Vec<usize>) {
        if let Some(process) = self.processes.get_mut(process_index) {  //Get process
            process.inputs = input_nets.iter().map(|net_index| {        //Get each input net index                                                                            
                if let Some(net) = self.nets.get_mut(*net_index) {      //Get net at index
                    net.add_sensitive_process(process_index);           //reg. process in net
                }
                Some(*net_index)
            }).collect();                                               //collect for input processes
        }
    }

    pub fn set_process_output(&mut self, process_index: usize, output_net: usize) {
        if let Some(process) = self.processes.get_mut(process_index) {
            process.output = Some(output_net);
        }
        if let Some(net) = self.nets.get_mut(output_net) {
            net.driver = Some(process_index);
        }
    }
    
    pub fn set_process_output_value(&mut self, process_index: usize, output_value: Value) {
        self.processes[process_index].output_value = output_value;
    }
    
    pub fn get_process_input_values(&self, process_index: usize) -> Vec<Value> {
        self.processes[process_index].inputs.iter().map(|net_index| {
            if let Some(index) = net_index {
                if let Some(driver_index) = self.nets[*index].driver {
                    return self.processes[driver_index].output_value;
                }
            } 
            Value::HIGHIMP
        }).collect() 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn state_test() {
        let mut state = State::new();
       
        let net_a = state.add_net();
        let net_b = state.add_net();
        let net_c = state.add_net();

        let process_a = state.add_process();
        let process_b = state.add_process();
        let process_c = state.add_process();
        let process_d = state.add_process();

        state.set_process_inputs(process_a, vec![net_c, net_b, net_a]);
        state.set_process_output(process_b, net_a);
        state.set_process_output(process_c, net_b);
        state.set_process_output(process_d, net_c);
        state.set_process_output_value(process_b, Value::ONE);
        state.set_process_output_value(process_c, Value::ZERO);
        state.set_process_output_value(process_d, Value::UNKNOWN);
        
        assert_eq!(state.processes[process_a].inputs.len(), 3);
        assert_eq!(state.processes[process_a].inputs[0], Some(net_c));
        assert_eq!(state.processes[process_a].inputs[1], Some(net_b));
        assert_eq!(state.processes[process_a].inputs[2], Some(net_a));
        assert_eq!(state.nets[net_a].sensitive_processes.len(), 1);
        assert_eq!(state.nets[net_b].sensitive_processes.len(), 1);
        assert_eq!(state.nets[net_c].sensitive_processes.len(), 1);
        assert_eq!(state.nets[net_a].sensitive_processes[0], process_a);
        assert_eq!(state.nets[net_b].sensitive_processes[0], process_a);
        assert_eq!(state.nets[net_c].sensitive_processes[0], process_a);

        assert_eq!(state.processes[process_b].output, Some(net_a));
        assert_eq!(state.processes[process_c].output, Some(net_b));
        assert_eq!(state.processes[process_d].output, Some(net_c));
        assert_eq!(state.nets[net_a].driver, Some(process_b));
        assert_eq!(state.nets[net_b].driver, Some(process_c));
        assert_eq!(state.nets[net_c].driver, Some(process_d));

        assert_eq!(state.processes[process_b].output_value, Value::ONE);
        assert_eq!(state.processes[process_c].output_value, Value::ZERO);
        assert_eq!(state.processes[process_d].output_value, Value::UNKNOWN);
        assert_eq!(state.get_process_input_values(process_a), vec![Value::UNKNOWN, Value::ZERO, Value::ONE]);
    }
}
