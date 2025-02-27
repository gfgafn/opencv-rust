pub mod barcode {
	//! # Barcode detecting and decoding methods
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::BarcodeDetectorTraitConst, super::BarcodeDetectorTrait };
	}
	
	pub const EAN_13: i32 = 2;
	pub const EAN_8: i32 = 1;
	pub const NONE: i32 = 0;
	pub const UPC_A: i32 = 3;
	pub const UPC_E: i32 = 4;
	pub const UPC_EAN_EXTENSION: i32 = 5;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum BarcodeType {
		NONE = 0,
		EAN_8 = 1,
		EAN_13 = 2,
		UPC_A = 3,
		UPC_E = 4,
		UPC_EAN_EXTENSION = 5,
	}
	
	opencv_type_enum! { crate::barcode::BarcodeType }
	
	/// Constant methods for [crate::barcode::BarcodeDetector]
	pub trait BarcodeDetectorTraitConst {
		fn as_raw_BarcodeDetector(&self) -> *const c_void;
	
		/// Detects Barcode in image and returns the rectangle(s) containing the code.
		/// 
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing (or not) Barcode.
		/// * points: Output vector of vector of vertices of the minimum-area rotated rectangle containing the codes.
		/// For N detected barcodes, the dimensions of this array should be [N][4].
		/// Order of four points in vector< Point2f> is bottomLeft, topLeft, topRight, bottomRight.
		#[inline]
		fn detect(&self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Decodes barcode in image once it's found by the detect() method.
		/// 
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing bar code.
		/// * points: vector of rotated rectangle vertices found by detect() method (or some other algorithm).
		/// For N detected barcodes, the dimensions of this array should be [N][4].
		/// Order of four points in vector<Point2f> is bottomLeft, topLeft, topRight, bottomRight.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * decoded_type: vector of BarcodeType, specifies the type of these barcodes
		#[inline]
		fn decode(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<crate::barcode::BarcodeType>) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_decode_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLBarcodeTypeGR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfBarcodeType(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Both detects and decodes barcode
		/// 
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing barcode.
		/// * decoded_info: UTF8-encoded output vector of string(s) or empty vector of string if the codes cannot be decoded.
		/// * decoded_type: vector of BarcodeType, specifies the type of these barcodes
		/// * points: optional output vector of vertices of the found  barcode rectangle. Will be empty if not found.
		/// 
		/// ## C++ default parameters
		/// * points: noArray()
		#[inline]
		fn detect_and_decode(&self, img: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<crate::barcode::BarcodeType>, points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detectAndDecode_const_const__InputArrayR_vectorLstringGR_vectorLBarcodeTypeGR_const__OutputArrayR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfBarcodeType(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::barcode::BarcodeDetector]
	pub trait BarcodeDetectorTrait: crate::barcode::BarcodeDetectorTraitConst {
		fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void;
	
	}
	
	pub struct BarcodeDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BarcodeDetector }
	
	impl Drop for BarcodeDetector {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_BarcodeDetector_delete(instance: *mut c_void); }
			unsafe { cv_BarcodeDetector_delete(self.as_raw_mut_BarcodeDetector()) };
		}
	}
	
	unsafe impl Send for BarcodeDetector {}
	
	impl crate::barcode::BarcodeDetectorTraitConst for BarcodeDetector {
		#[inline] fn as_raw_BarcodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::barcode::BarcodeDetectorTrait for BarcodeDetector {
		#[inline] fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BarcodeDetector {
		/// Initialize the BarcodeDetector.
		/// ## Parameters
		/// * prototxt_path: prototxt file path for the super resolution model
		/// * model_path: model file path for the super resolution model
		/// 
		/// ## C++ default parameters
		/// * prototxt_path: ""
		/// * model_path: ""
		#[inline]
		pub fn new(prototxt_path: &str, model_path: &str) -> Result<crate::barcode::BarcodeDetector> {
			extern_container_arg!(prototxt_path);
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(prototxt_path.opencv_as_extern(), model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::barcode::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
}
