use ndarray::Array2;

#[derive(Debug)]
pub struct LogEntry
{
    pub ip: String,
    pub status: f64,
    pub size: f64,
}

impl LogEntry
 {
    pub fn to_matrix(logs: &[LogEntry]) -> Array2<f64> 
    {
        let mut matrix = Array2::zeros((logs.len(), 2));

        for (i, log) in logs.iter().enumerate() 
        {
            matrix[[i, 0]] = log.status;
            matrix[[i, 1]] = log.size;
        }
        return matrix;   

    }

    pub fn normalize_matrix(matrix: &mut Array2<f64>) 
    {
        for i in 0..matrix.ncols()
        {
            let min = matrix.column(i).iter().cloned().fold(f64::INFINITY, f64::min);
            let max = matrix.column(i).iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            if max - min == 0.0 
            {
                matrix.column_mut(i).mapv_inplace(|_x| 0.0);
            }
            else
            {
                matrix.column_mut(i).mapv_inplace(|x| (x - min) / (max - min));
            }
        }
    }
}
