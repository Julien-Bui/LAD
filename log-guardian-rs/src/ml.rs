use ndarray::{Array2, Axis};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa::traits::{Fit, Predict};

pub fn detect_anomalies(matrix: &Array2<f64>, threshold: f64) -> Vec<bool> 
{
    
    let dataset = DatasetBase::from(matrix.clone());

    
    let model = KMeans::params(3)
        .fit(&dataset)
        .expect("KMeans fitting failed");

   
    let predictions = model.predict(dataset);
    let targets = predictions.targets();

    
    let mut counts = vec![0usize; 3];
    for &cluster_id in targets.iter() 
    {
        counts[cluster_id] += 1;
    }

    
    let normal_cluster = counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map(|(index, _)| index)
        .unwrap();

    
    let centroids = model.centroids();
    let normal_centroid = centroids.index_axis(Axis(0), normal_cluster);

    
    let mut anomalies = vec![false; matrix.nrows()];
    for i in 0..matrix.nrows() 
    {
        let distance = ((matrix[[i, 0]] - normal_centroid[0]).powi(2)+ (matrix[[i, 1]] - normal_centroid[1]).powi(2)).sqrt();
        anomalies[i] = distance > threshold;
    }

    anomalies
}
